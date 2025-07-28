use std::path::{Path, PathBuf};

use walkdir::{DirEntry, IntoIter, WalkDir};

use crate::utils::display_error_and_exit;

/// Recursively walks the folders in a path looking for empties
pub struct EmptiesWalker<'a> {
    /// Path to recursively walk
    pub path: PathBuf,

    /// Show empty hidden folders switch
    pub show_hidden: bool,

    /// `WalkDir` iterator
    pub walker: IntoIter,
    pub skipped: &'a [&'a str],
}

impl Iterator for EmptiesWalker<'_> {
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

                    let folder_name = entry.file_name().to_string_lossy();
                    let git_folder = folder_name == ".git";

                    // skip git folders
                    if git_folder {
                        self.walker.skip_current_dir();
                        continue;
                    }

                    let hidden_folder = folder_name.starts_with('.');

                    // skip hidden entries if skip empty hidden entry
                    if hidden_folder && !self.show_hidden {
                        self.walker.skip_current_dir();
                        continue;
                    }

                    if self.skipped.iter().any(|skip| folder_name == *skip) {
                        self.walker.skip_current_dir();
                        continue;
                    }

                    let empty = self.is_folder_empty(&entry);

                    // hidden or not iterate empty entry
                    if empty.is_some() {
                        self.walker.skip_current_dir();

                        return empty;
                    }

                    // skip hidden entry that is not empty
                    if hidden_folder {
                        self.walker.skip_current_dir();
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

impl<'a> EmptiesWalker<'a> {
    pub fn new<P: AsRef<Path>>(
        path: P, show_hidden: bool,
        skipped: &'a [&'a str],
    ) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            show_hidden,
            walker: WalkDir::new(path).into_iter(),
            skipped,
        }
    }

    /// Determines if entry is an empty folder
    fn is_folder_empty(&self, entry: &DirEntry) -> Option<PathBuf> {
        let path = entry.path();

        if WalkDir::new(path).into_iter().any(|e| match e {
            Ok(e) => e.file_type().is_file(),
            Err(err) => {
                display_error_and_exit(&format!(
                    "Exception while searching \"{}\" for empties: {err}",
                    self.path.to_string_lossy()
                ));
            }
        }) {
            None
        } else {
            Some(path.to_path_buf())
        }
    }
}
