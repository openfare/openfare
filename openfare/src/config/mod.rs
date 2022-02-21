use anyhow::Result;

mod core;
mod extensions;
mod paths;
mod profile;
mod services;

pub use paths::Paths;

#[derive(
    Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Config {
    pub core: core::Core,
    pub services: services::Services,
    pub profile: profile::Profile,
    pub extensions: extensions::Extensions,
}

impl crate::common::json::Subject<Config> for Config {
    fn subject(&self) -> &Self {
        &self
    }
    fn subject_mut(&mut self) -> &mut Self {
        self
    }
}

impl crate::common::fs::FilePath for Config {
    fn file_path() -> Result<std::path::PathBuf> {
        let paths = paths::Paths::new()?;
        Ok(paths.config_file)
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}
