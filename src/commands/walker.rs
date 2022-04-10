use std::fs::read_dir;

use walkdir::{DirEntry, IntoIter, WalkDir};

use crate::{AllValues, Platform};
use crate::models::BuildArtifacts;

/// Recursively walks the folders in a path looking for build artifacts
pub struct BuildsWalker<'a> {
    /// Filters supported platforms to include in iteration
    pub filter: &'a AllValues,

    /// Path to recursively walk
    pub path: &'a str,

    /// All supported platforms
    pub platforms: &'a [Platform<'a>],

    /// WalkDir iterator
    pub walker: IntoIter,
}

impl<'a> Iterator for BuildsWalker<'a> {
    type Item = BuildArtifacts<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.walker.next()?;

            match next {
                Ok(entry) => {
                    if let Some(artifacts) = self.build_artifacts(&entry) {
                        return Some(artifacts);
                    }
                }
                Err(err) => {
                    eprint!("Exception while searching \"{}\" for build artifacts: {err}", self.path);

                    break;
                }
            }
        }

        None
    }
}

impl<'a> BuildsWalker<'a> {
    pub fn new(filter: &'a AllValues, path: &'a str, platforms: &'a [Platform]) -> Self {
        Self {
            filter,
            path,
            platforms,
            walker: WalkDir::new(path).into_iter(),
        }
    }

    /// Determines if entry is matches a supported platform with build artifacts
    fn build_artifacts(&self, entry: &DirEntry) -> Option<BuildArtifacts<'a>> {
        if entry.file_type().is_dir() {
            let folder = entry.file_name().to_string_lossy().to_lowercase();
            let parent = entry.path().parent().unwrap().to_string_lossy().to_lowercase();

            for platform in self.platforms.iter()
                .filter(|p| self.filter.matches(p.name) && p.folders.contains(&folder.as_str()))
            {
                if parent.contains(folder.as_str()) {
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

                            // todo: support wildcards instead
                            platform.associated.contains(&file_name.as_str()) ||
                                platform.associated.contains(&ext.as_str())
                        } else {
                            false
                        }
                    ).count() > 0 {
                        return Some(BuildArtifacts {
                            name: platform.name,
                            folder: entry.path().to_string_lossy().to_string(),
                        });
                    }
                }
            }
            None
        } else {
            None
        }
    }
}
