use std::fs::read_dir;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, IntoIter, WalkDir};

use crate::models::BuildArtifacts;
use crate::utils::display_error_and_exit;
use crate::{Platform, Selection};

/// Recursively walks the folders in a path looking for build artifacts
pub struct BuildsWalker<'a> {
    /// Filters supported platforms to include in iteration
    pub filter: &'a Selection,

    /// Path to recursively walk
    pub path: PathBuf,

    /// All supported platform filter
    pub platforms: &'a [Platform],

    /// `WalkDir` iterator
    pub walker: IntoIter,
}

impl<'a> Iterator for BuildsWalker<'a> {
    type Item = BuildArtifacts<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.walker.next()?;

            match next {
                Ok(entry) => {
                    let artifacts = self.build_artifacts(&entry);

                    if artifacts.is_some() {
                        self.walker.skip_current_dir();

                        return artifacts;
                    }
                }
                Err(err) => {
                    display_error_and_exit(&format!(
                        "Exception while searching \"{}\" for build artifacts: {err}",
                        self.path.to_string_lossy()
                    ));
                }
            }
        }
    }
}

impl<'a> BuildsWalker<'a> {
    pub fn new<P: AsRef<Path>>(filter: &'a Selection, path: P, platforms: &'a [Platform]) -> Self {
        Self {
            filter,
            path: path.as_ref().to_path_buf(),
            platforms,
            walker: WalkDir::new(path).into_iter(),
        }
    }

    /// Determines if entry matches a supported platform with build artifacts
    fn build_artifacts(&self, entry: &DirEntry) -> Option<BuildArtifacts<'a>> {
        if entry.file_type().is_dir() {
            let folder = entry.file_name().to_string_lossy().to_lowercase();
            let parent = entry.path().parent().unwrap().to_string_lossy().to_lowercase();

            for platform in self.platforms.iter()
                .filter(|p| self.filter.matches(&p.name) && p.folders.contains(&folder))
            {
                if parent.contains(folder.as_str()) {
                    break;
                }

                if let Ok(files) = read_dir(&parent) {
                    if files
                        .filter(|v| {
                            v.as_ref().is_ok_and(|file| {
                                let file_name = file.file_name().to_string_lossy().to_lowercase();

                                platform.associated.iter().any(|f| f.matches(&file_name))
                            })
                        })
                        .count()
                        > 0
                    {
                        return Some(BuildArtifacts {
                            name: &platform.name,
                            folder: entry.path().to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }

        None
    }
}
