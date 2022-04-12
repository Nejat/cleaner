/// Describes a supported development platform
#[derive(Serialize, Deserialize)]
pub struct Platform {
    /// Development platform name
    pub name: String,

    /// Expected build artifact folder names of platform
    pub folders: Vec<String>,

    /// Associated files and file extensions that mark the platform
    pub associated: Vec<String>,
}

impl AsRef<str> for Platform {
    fn as_ref(&self) -> &str {
        &self.name
    }
}
