use std::path::{Path, PathBuf};

use git2::{Error, Repository};
use walkdir::{DirEntry, IntoIter, WalkDir};

use crate::utils::display_error_and_exit;

/// Recursively walks the folders in a path looking for empties
pub struct ReposWalker {
    /// Path to recursively walk
    pub path: PathBuf,

    /// `WalkDir` iterator
    pub walker: IntoIter,
}

impl Iterator for ReposWalker {
    type Item = (Result<Repository, Error>, PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.walker.next()?;

            match next {
                Ok(entry) => {
                    // only process folders
                    if entry.file_type().is_file() {
                        continue;
                    }

                    if let Some(repo_path) = self.is_folder_a_repo(&entry) {
                        self.walker.skip_current_dir();

                        return Some((Repository::open(&repo_path), repo_path));
                    }
                }
                Err(err) => {
                    display_error_and_exit(&format!(
                        "Exception while searching \"{}\" for empties: {err}",
                        self.path.to_string_lossy()
                    ));
                }
            }
        }
    }
}

impl ReposWalker {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            walker: WalkDir::new(path).into_iter(),
        }
    }

    /// Determines if entry is a git repository folder
    fn is_folder_a_repo(&self, entry: &DirEntry) -> Option<PathBuf> {
        let path = entry.path();
        let walker = WalkDir::new(path).min_depth(1).max_depth(1);

        if walker.into_iter().any(|e| match e {
            Ok(e) => {
                e.file_type().is_dir() && e.file_name() == ".git"
            }
            Err(err) => {
                display_error_and_exit(&format!(
                    "Exception while searching \"{}\" for git repositories: {err}",
                    self.path.to_string_lossy()
                ));
            }
        }) {
            Some(path.to_path_buf())
        } else {
            None
        }
    }
}
