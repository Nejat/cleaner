/// Describes an existing build artifact
pub struct BuildArtifacts<'a> {
    /// Development platform name
    pub name: &'a str,

    /// Build artifact folder located
    pub folder: String,
}
