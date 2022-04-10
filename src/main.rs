#![doc = include_str!("../README.md")]

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/cleaner/0.1.0")]

use std::env;
use std::fs::{read_dir, remove_dir_all};
use std::path::PathBuf;
use std::process::exit;

#[cfg(test)]
mod tests;

/// Describes a support project type for cleaning
#[doc(hidden)]
struct Project {
    /// Type of project
    name: &'static str,

    /// Build artifact folders of project type
    folders: Vec<&'static str>,

    /// Files and file extensions that marks the project type
    associated: Vec<&'static str>,
}

#[doc(hidden)]
fn main() {
    const USAGE: &str = "usage: clean-dist <path> [-forreals]\n";

    let mut args = env::args().into_iter().skip(1);

    let path = match args.next() {
        None => {
            println!("{}", USAGE);
            exit(-1);
        }
        Some(path) => {
            let path = PathBuf::from(path);

            if !path.exists() {
                println!("path: \"{}\" - does not exist!\n", path.to_string_lossy());
                exit(-1);
            }

            if path.is_file() {
                println!("path: \"{}\" - is not directory!\n", path.to_string_lossy());
                exit(-1);
            }

            path
        }
    };

    let for_reals = match args.next() {
        None => false,
        Some(for_reals) if for_reals.to_lowercase() == "-forreals" => true,
        Some(_) => {
            println!("{}", USAGE);
            exit(-1);
        }
    };

    let projects = vec![
        Project {
            name: ".Net",
            folders: vec!["bin", "obj"],
            associated: vec!["sln", "csproj"],
        },
        Project {
            name: "Rust",
            folders: vec!["target"],
            associated: vec!["cargo.toml"],
        },
        Project {
            name: "Web",
            folders: vec!["node_modules"],
            associated: vec!["package.json"],
        },
    ];

    let mut found = 0;

    for entry in walkdir::WalkDir::new(path) {
        let entry = entry.unwrap();

        if entry.file_type().is_dir() {
            let folder = entry.file_name().to_string_lossy().to_lowercase();
            let parent = entry.path().parent().unwrap().to_string_lossy().to_lowercase();

            for project in projects.iter().filter(|f| f.folders.contains(&folder.as_str())) {
                if parent.contains(&folder.as_str()) {
                    break;
                }

                if let Ok(files) = read_dir(&parent) {
                    if files.filter(
                        |v| if let Ok(file) = v {
                            let file_name = file.file_name().to_string_lossy().to_lowercase();
                            let ext = match file.path().extension() {
                                None => String::default(),
                                Some(ext) => ext.to_string_lossy().to_lowercase()
                            };

                            project.associated.contains(&file_name.as_str()) ||
                                project.associated.contains(&ext.as_str())
                        } else {
                            false
                        }
                    ).count() > 0 {
                        found += 1;

                        if for_reals {
                            match remove_dir_all(entry.path()) {
                                Ok(_) =>
                                    println!("[{:04}] {} - {}", found, project.name, parent),
                                Err(err) =>
                                    println!("ERR: {} - {}: {}", project.name, entry.path().to_string_lossy(), err)
                            }
                        } else {
                            println!("[{:04}] {} - {}", found, project.name, entry.path().to_string_lossy());
                        }
                    }
                }
            }
        }
    }
}
