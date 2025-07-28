use std::path::{Path, PathBuf};

use walkdir::{DirEntry, IntoIter, WalkDir};

use crate::utils::display_error_and_exit;

/// Recursively walks the folders in a path looking for empties
pub struct EmptiesWalker {
    /// Path to recursively walk
    pub path: PathBuf,

    /// Show empty hidden folders switch
    pub show_hidden: bool,

    /// `WalkDir` iterator
    pub walker: IntoIter,
}

impl Iterator for EmptiesWalker {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.walker.next()?;

            match next {
                Ok(entry) => {
                    // only process folders
                    if entry.file_type().is_file() {
                        continue;
                    }

                    let hidden_folder = entry.path()
                        .file_name()
                        .is_some_and(|p| p.to_str().is_some_and(|s| s.starts_with('.')));

                    // skip hidden entries if skip empty hidden entry
                    if hidden_folder && !self.show_hidden {
                        self.walker.skip_current_dir();
                        continue;
                    }

                    let empties = self.is_folder_empty(&entry);

                    // hidden or not iterate empty entry
                    if empties.is_some() {
                        self.walker.skip_current_dir();

                        return empties;
                    }

                    // skip hidden entry that is not empty
                    if hidden_folder {
                        self.walker.skip_current_dir();
                    }
                }
                Err(err) => {
                    display_error_and_exit(
                        &format!("Exception while searching \"{}\" for empties: {err}", self.path.to_string_lossy())
                    );
                }
            }
        }
    }
}

impl EmptiesWalker {
    pub fn new<P: AsRef<Path>>(path: P, show_hidden: bool) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            show_hidden,
            walker: WalkDir::new(path).into_iter(),
        }
    }

    /// Determines if entry is an empty folder
    fn is_folder_empty(&self, entry: &DirEntry) -> Option<PathBuf> {
        let path = entry.path();

        if WalkDir::new(path).into_iter().any(
            |e| match e {
                Ok(e) => e.file_type().is_file(),
                Err(err) => {
                    display_error_and_exit(
                        &format!("Exception while searching \"{}\" for empties: {err}", self.path.to_string_lossy())
                    );
                }
            }
        ) {
            None
        } else {
            Some(path.to_path_buf())
        }
    }
}
