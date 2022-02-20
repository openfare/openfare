use anyhow::Result;

/// Filesystem config directory absolute paths.
#[derive(Debug)]
pub struct Paths {
    pub root_directory: std::path::PathBuf,
    pub config_file: std::path::PathBuf,
    pub profile_file: std::path::PathBuf,
    pub extensions_directory: std::path::PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self> {
        let user_directories = directories::ProjectDirs::from("", "", "openfare").ok_or(
            anyhow::format_err!("Failed to obtain a handle on the local user directory."),
        )?;
        let root_directory = user_directories.config_dir();
        Ok(Self {
            root_directory: root_directory.into(),
            config_file: root_directory.join("config.json"),
            profile_file: root_directory.join("profile.json"),
            extensions_directory: root_directory.join("extensions"),
        })
    }
}
