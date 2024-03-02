use std::path::Path;

use crate::cli::commands::repos::OutdatedFilter;

pub fn list_outdated_repos<P: AsRef<Path>>(_path: P, _filter: OutdatedFilter, _only_main: bool) {}

pub fn list_repos_that_are_branched<P: AsRef<Path>>(_path: P) {}

pub fn list_repos_that_are_init_only<P: AsRef<Path>>(_path: P) {}

pub fn list_repos_with_branch<P: AsRef<Path>>(_path: P, _branch_name: &str) {}

pub fn list_repos_with_uncommitted_changes<P: AsRef<Path>>(_path: P) {}

pub fn list_repos_without_configured_remotes<P: AsRef<Path>>(_path: P) {}

pub fn list_repos_with_detached_head<P: AsRef<Path>>(_path: P) {}

pub fn list_repos_with_errors<P: AsRef<Path>>(_path: P) {}

pub fn list_up_to_date_repos<P: AsRef<Path>>(_path: P, _only_main: bool) {}
