use anyhow::{Context, Result};

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
fn set_directory_hidden_windows(_directory: &std::path::PathBuf) {
    // TODO: Hide directory on Windows.
    // winapi::um::fileapi::SetFileAttributesA()
}

#[cfg(not(windows))]
fn set_directory_hidden_windows(_directory: &std::path::PathBuf) {}

pub trait FilePath {
    fn file_path() -> Result<std::path::PathBuf>;
}

pub trait FileStore: FilePath {
    fn load() -> Result<Self>
    where
        Self: Sized;
    fn dump(&mut self) -> Result<()>;
}

impl<'de, T> FileStore for T
where
    T: FilePath + Default + serde::de::DeserializeOwned + serde::Serialize,
{
    fn load() -> Result<Self> {
        if !Self::file_path()?.is_file() {
            let mut default = Self::default();
            default.dump()?;
        }

        let file = std::fs::File::open(Self::file_path()?)?;
        let reader = std::io::BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    fn dump(&mut self) -> Result<()> {
        if Self::file_path()?.is_file() {
            std::fs::remove_file(&Self::file_path()?)?;
        }

        let file = std::fs::OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(&Self::file_path()?)
            .context(format!(
                "Can't open/create file for writing: {}",
                Self::file_path()?.display()
            ))?;

        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }
}
