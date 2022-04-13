use std::path::PathBuf;

use walkdir::{DirEntry, IntoIter, WalkDir};

use crate::utils::display_error_and_exit;

/// Recursively walks the folders in a path looking for empties
pub struct EmptiesWalker<'a> {
    /// Path to recursively walk
    pub path: &'a str,

    /// Show empty hidden folders switch
    pub show_hidden: bool,

    /// WalkDir iterator
    pub walker: IntoIter,
}

impl<'a> Iterator for EmptiesWalker<'a> {
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
                        .map_or(false, |p| p.to_str().map_or(false, |s| s.starts_with('.')));

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
                        continue;
                    }
                }
                Err(err) => {
                    display_error_and_exit(
                        &format!("Exception while searching \"{}\" for empties: {err}", self.path)
                    );
                }
            }
        }
    }
}

impl<'a> EmptiesWalker<'a> {
    pub fn new(path: &'a str, show_hidden: bool) -> Self {
        Self {
            path,
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
                        &format!("Exception while searching \"{}\" for empties: {err}", self.path)
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
