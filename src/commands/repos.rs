#![allow(unused_qualifications)] // OnError is necessary for no on error handlers

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use git2::string_array::StringArray;
use git2::{
    Branch, BranchType, Branches, Config, Cred, Direction, Error, ErrorCode, FetchOptions,
    Reference, Remote, RemoteCallbacks, Repository, StatusOptions,
};
use rayon::prelude::*;
use regex::Regex;

use crate::cli::commands::repos::OutdatedFilter;
use crate::commands::walkers::ReposWalker;
use crate::models::BranchName;
use crate::utils::validate_path;

type OnError = Option<Box<dyn Fn(Error, &'_ str) -> bool + Sync>>;
type Message = Option<String>;

// outputs errors for repos, with and optional message prefix (only relevant in this module)
macro_rules! output_err {
    ($err: ident, $path: expr) => {
        if $err.code() != ErrorCode::UnbornBranch {
            let path = $path;
            let err = $err.to_string().replace(&format!(" at '{path}'"), "");

            println!("{path} - Err: {err}");
        }
    };
    ($err: ident, $path: expr, $msg: tt) => {
        if $err.code() != ErrorCode::UnbornBranch {
            let path = $path;
            let err = $err.to_string().replace(&format!(" at '{path}'"), "");

            println!("{} {path} - Err: {err}", $msg);
        }
    };
}

// macrofies project's mundane error specifically working with repos (only valid in this module)
macro_rules! ok {
    // default, continue loop
    ($result: expr) => {
        ok!(@let $result, continue)
    };
    ($result: expr, $path: expr) => {
        ok!($result, $path, continue)
    };
    // exit failed on error
    (fail; $result: expr, $path: expr) => {
        ok!($result, $path, return false)
    };
    // bail on error
    (bail; $result: expr, $path: expr) => {
        ok!($result, $path, return)
    };
    // repo handler; exit false, none
    (hndlr; $result: expr, $path: expr $(, $msg: tt)?) => {
        ok!($result, $path, return (false, Message::None) $(, $msg)?)
    };
    // option handler; exit None on error
    (opt; $result: expr, $path: expr) => {
        ok!($result, $path, return None)
    };
    // check if ok
    (ok; $result: expr, $path: expr $(, $msg: tt)?) => {
        match $result {
            Ok(ok) => ok,
            Err(err) => {
                output_err!(err, $path $(, $msg)?);

                return Err(err);
            }
        }
    };
    // check if error
    (err; $result: expr, $path: expr $(, $msg: tt)?) => {
        match $result {
            Ok(_) => {}
            Err(err) => {
                output_err!(err, $path $(, $msg)?);

                return Err(err);
            }
        }
    };
    // generic implementation
    ($result: expr, $path: expr, $next: stmt $(, $msg: tt)?) => {
        match $result {
            Ok(ok) => ok,
            Err(err) => {
                output_err!(err, $path $(, $msg)?);

                $next
            }
        }
    };
    // generic implementation
    (@let $result: expr, $next: stmt) => {
        if let Ok(ok) = $result { ok } else { $next }
    };
}

static RE_MAIN_OR_MASTER: once_cell::sync::OnceCell<regex::Regex> =
    once_cell::sync::OnceCell::new();

pub fn list_repos<P: AsRef<Path>>(path: P) {
    repos_handler(
        path,
        |_, _| (true, Message::None),
        OnError::None.as_ref(),
        || "Did not find any repos",
    );
}

pub fn list_repos_with_detached_head<P: AsRef<Path>>(path: P) {
    repos_handler(
        path,
        |repo, repo_path| (ok!(hndlr; repo.head_detached(), repo_path), Message::None),
        OnError::None.as_ref(),
        || "Did not find any detached repos",
    );
}

pub fn list_repos_with_errors<P: AsRef<Path>>(path: P) {
    let handler = |err: Error, repo_path: &'_ str| {
        output_err!(err, repo_path);

        true
    };
    repos_handler(
        path,
        |_, _| (false, Message::None),
        Some(&handler),
        || "Did not find any repos with errors",
    );
}

pub fn list_repos_that_are_branched<P: AsRef<Path>>(path: P) {
    repos_handler(
        path,
        |repo, repo_path| {
            repo_is_branched(repo, repo_path)
                .map_or((false, Message::None), |branched| (branched, Message::None))
        },
        OnError::None.as_ref(),
        || "Did not find any repos that are in branch",
    );
}

pub fn list_repos_that_are_init_only<P: AsRef<Path>>(path: P) {
    repos_handler(
        path,
        |repo, repo_path| {
            let init_only = repo_is_init_only(repo);

            if init_only {
                println!("{repo_path}");
            }

            (init_only, Message::None)
        },
        OnError::None.as_ref(),
        || "Did not find any init only repos",
    );
}

pub fn list_repos_with_branch<P: AsRef<Path>>(path: P, branch_name: &str) {
    repos_handler(
        path,
        |repo, repo_path| {
            let branch = find_local_branch(repo, |name| name == branch_name);
            let branch = ok!(hndlr; branch, repo_path);

            (branch.is_some(), Message::None)
        },
        OnError::None.as_ref(),
        || format!("Did not find any repos with a \"{branch_name}\" branch"),
    );
}

pub fn list_repos_with_uncommitted_changes<P: AsRef<Path>>(path: P) {
    repos_handler(
        path,
        |repo, repo_path| {
            let mut options = StatusOptions::new();
            let mut options = options.include_untracked(true);
            let changes = ok!(hndlr; repo.statuses(Some(&mut options)), repo_path)
                .iter()
                .count();

            (changes > 0, Some(format!("changes: {changes}")))
        },
        OnError::None.as_ref(),
        || "Did not find any repos with uncommitted changes",
    );
}

pub fn list_repos_without_configured_remotes<P: AsRef<Path>>(path: P) {
    repos_handler(
        path,
        |repo, repo_path| {
            let remotes = ok!(hndlr; repo.remotes(), repo_path);
            let has_remotes = remotes.is_empty() && !repo_is_init_only(repo);

            (has_remotes, Message::None)
        },
        OnError::None.as_ref(),
        || "Did not find any repos with out remotes",
    );
}

pub fn list_outdated_repos<P: AsRef<Path>>(path: P, filter: OutdatedFilter, only_main: bool) {
    repos_handler(
        path,
        |repo, repo_path| {
            ok!(hndlr; fetch_all_remotes(repo, repo_path), repo_path);

            let branches = ok!(hndlr; repo.branches(Some(BranchType::Local)), repo_path);
            let branches = filter_main_branches(branches, only_main);

            let result =
                check_branch_status(branches, repo, repo_path, |branch, upstream| match filter {
                    OutdatedFilter::Ahead => branch > upstream,
                    OutdatedFilter::Either => branch != upstream,
                    OutdatedFilter::Behind => branch < upstream,
                })
                .any(|found| found);

            (result, Message::None)
        },
        OnError::None.as_ref(),
        || "Did not find any outdated repos",
    );
}

pub fn list_up_to_date_repos<P: AsRef<Path>>(path: P, only_main: bool) {
    repos_handler(
        path,
        |repo, repo_path| {
            ok!(hndlr; fetch_all_remotes(repo, repo_path), repo_path);

            let branches = ok!(hndlr; repo.branches(Some(BranchType::Local)), repo_path);
            let branches = filter_main_branches(branches, only_main);

            let result = check_branch_status(branches, repo, repo_path, |branch, upstream| {
                branch == upstream
            })
            .all(|found| found);

            (result, Message::None)
        },
        OnError::None.as_ref(),
        || "Did not find any up-to-date repos",
    );
}

fn fetch_all_remotes(repo: &Repository, repo_path: &str) -> Result<(), Error> {
    let remotes = ok!(ok; repo.remotes(), repo_path, "Couldn't get remotes");
    let remotes = authenticated_remotes(&remotes, repo, repo_path);

    for mut remote in remotes {
        let mut fetch_options = FetchOptions::new();

        fetch_options.remote_callbacks(credentials());

        ok!(err; remote.fetch(
            &[] as &[&str], Some(&mut fetch_options), None
        ), repo_path, "Couldn't fetch remote");
    }

    Ok(())
}

fn check_branch_status<'a>(
    branches: impl IntoIterator<Item = Result<(Branch<'a>, BranchType), Error>> + 'a,
    repo: &'a Repository,
    repo_path: &'a str,
    predicate: impl Fn(usize, usize) -> bool + 'a,
) -> impl Iterator<Item = bool> + 'a {
    branches.into_iter().filter_map(move |branch| {
        let (branch, _) = branch.ok()?;
        let upstream = branch.upstream().ok()?;
        let oid = branch.into_reference().target()?;
        let upstream_oid = upstream.into_reference().target()?;
        let (branch, upstream) = ok!(opt; repo.graph_ahead_behind(oid, upstream_oid), repo_path);

        Some(predicate(branch, upstream))
    })
}

fn authenticated_remotes<'a>(
    remotes: &'a StringArray,
    repo: &'a Repository,
    repo_path: &'a str,
) -> impl Iterator<Item = Remote<'a>> {
    remotes.into_iter().filter_map(move |remote_url| {
        let mut remote = repo.find_remote(remote_url.unwrap_or_default()).ok()?;

        ok!(opt; remote.connect_auth(
                Direction::Fetch, Some(credentials()), None
            ), repo_path);

        Some(remote)
    })
}

fn credentials<'a>() -> RemoteCallbacks<'a> {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|url, usr, _| {
        let config = get_config().unwrap();

        Cred::credential_helper(&config, url, usr)
    });

    callbacks
}

fn filter_main_branches(
    branches: Branches,
    only_main: bool,
) -> impl Iterator<Item = Result<(Branch<'_>, BranchType), Error>> {
    branches.filter(move |branch| {
        if only_main && !branch.is_err() {
            let main_branch = re_main_or_master();

            // just checked branch is not err
            let (branch, _) = branch.as_ref().unwrap();

            let branch_name = branch.name().unwrap_or_default().unwrap_or_default();

            main_branch.is_match(branch_name)
        } else {
            true
        }
    })
}

fn find_local_branch<'a, P>(repo: &'a Repository, predicate: P) -> Result<Option<Branch<'a>>, Error>
where
    P: Fn(&'_ str) -> bool + 'a + Copy,
{
    let mut branches = repo.branches(Some(BranchType::Local))?;

    let branch = branches.find(|branch| {
        let Ok((branch, _)) = branch else {
            return false;
        };

        branch.name().unwrap_or_default().is_some_and(predicate)
    });

    match branch {
        None => Ok(None),
        Some(Ok((branch, _))) => Ok(Some(branch)),
        Some(Err(err)) => Err(err),
    }
}

fn get_branch_name<'a>(reference: &'a Reference) -> BranchName<'a> {
    let name = reference.shorthand().unwrap_or_default();

    if name == "HEAD" {
        if let Some(oid) = reference.target() {
            return BranchName::Head(oid);
        }
    }

    BranchName::Branch(name)
}

fn get_config() -> Result<Config, Error> {
    if let Ok(config) = Config::open_default() {
        Ok(config)
    } else {
        let config_path: PathBuf = match Config::find_xdg() {
            Ok(path) => path,
            Err(_) => match Config::find_system() {
                Ok(path) => path,
                Err(_) => Config::find_global()?,
            },
        };

        Config::open(config_path.as_path())
    }
}

fn re_main_or_master() -> &'static Regex {
    RE_MAIN_OR_MASTER.get_or_init(|| Regex::new("^(refs/heads/)?(main|master)$").unwrap())
}

fn repos_handler<P, S, H, OE, E, NF>(path: P, handler: H, on_error: Option<&OE>, not_found: NF)
where
    P: AsRef<Path>,
    S: AsRef<str>,
    H: Fn(&'_ Repository, &'_ str) -> (bool, Option<S>) + Sync,
    OE: Fn(Error, &'_ str) -> bool + Sync,
    E: AsRef<str>,
    NF: Fn() -> E,
{
    validate_path(&path);

    let found = AtomicBool::new(false);

    ReposWalker::new(&path)
        .par_bridge()
        .for_each(|(repo, repo_path)| {
            let repo_path = repo_path.to_string_lossy();

            let repo = match repo {
                Ok(repo) => repo,
                Err(err) => {
                    if let Some(on_error) = on_error {
                        if on_error(err, &repo_path) {
                            found.store(true, Ordering::Relaxed);
                        }
                    }

                    return;
                }
            };

            let (found_repo, message) = handler(&repo, &repo_path);

            if found_repo {
                found.store(true, Ordering::Relaxed);

                let repo_head = &ok!(bail; repo.head(), repo_path);
                let branch_name = get_branch_name(repo_head);

                let message =
                    message.map_or_else(String::new, |message| format!("; {}", message.as_ref()));

                println!("{repo_path} - {branch_name}{message}");
            }
        });

    if !found.load(Ordering::Relaxed) {
        println!("{}", not_found().as_ref());
    }
}

fn repo_is_branched(repo: &Repository, repo_path: &str) -> Option<bool> {
    let main_branch = re_main_or_master();

    Some(
        ok!(opt; repo.head(), repo_path)
            .name()
            .is_some_and(|head_name| !main_branch.is_match(head_name)),
    )
}

fn repo_is_init_only(repo: &Repository) -> bool {
    if let Err(err) = repo.head() {
        err.code() == ErrorCode::UnbornBranch
    } else {
        false
    }
}
