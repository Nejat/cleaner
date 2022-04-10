/// Describes a supported development platform
pub struct Platform<'a> {
    /// Development platform name
    pub name: &'a str,

    /// Expected build artifact folder names of platform
    pub folders: Vec<&'a str>,

    /// Associated files and file extensions that mark the platform
    pub associated: Vec<&'a str>,
}
