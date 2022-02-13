use anyhow::{format_err, Result};
use directories;

pub mod archive;

pub fn ensure_extensions_bin_directory() -> Result<Option<std::path::PathBuf>> {
    // Attempt to create an extensions directory in the users home directory.
    let extensions_directory = get_extensions_default_directory();

    // Use user local bin if previous path is None.
    let extensions_directory = extensions_directory.or(dirs::executable_dir());

    // Ensure directory exists.
    if let Some(extensions_directory) = &extensions_directory {
        if !extensions_directory.exists() {
            log::debug!(
                "Creating OpenFare extensions bin directory: {}",
                extensions_directory.display()
            );
            std::fs::create_dir_all(&extensions_directory)?;
            set_directory_hidden_windows(&extensions_directory);
        }
    }
    Ok(extensions_directory)
}

/// Does not create the directory.
/// Returns None if home directory does not exist.
pub fn get_extensions_default_directory() -> Option<std::path::PathBuf> {
    let extensions_directory_name = ".openfare_extensions";

    match dirs::home_dir() {
        Some(home_directory) => {
            if !home_directory.exists() {
                None
            } else {
                let extensions_directory = home_directory.join(extensions_directory_name);
                Some(extensions_directory)
            }
        }
        None => None,
    }
}

#[cfg(windows)]
fn set_directory_hidden_windows(directory: &std::path::PathBuf) {
    // TODO: Hide directory on Windows.
    // winapi::um::fileapi::SetFileAttributesA()
}

#[cfg(not(windows))]
fn set_directory_hidden_windows(_directory: &std::path::PathBuf) {}

/// Filesystem config directory absolute paths.
#[derive(Debug)]
pub struct ConfigPaths {
    pub root_directory: std::path::PathBuf,
    pub config_file: std::path::PathBuf,
    pub profile_file: std::path::PathBuf,
    pub extensions_directory: std::path::PathBuf,
}

impl ConfigPaths {
    pub fn new() -> Result<Self> {
        let user_directories = directories::ProjectDirs::from("", "", "openfare").ok_or(
            format_err!("Failed to obtain a handle on the local user directory."),
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
