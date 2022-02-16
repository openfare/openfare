use anyhow::{format_err, Result};

pub mod common;
mod core;
mod extensions;
mod portal;
mod profile;

pub use common::{FilePath, FileStore};

#[derive(
    Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Config {
    pub core: core::Core,
    pub portal: portal::Portal,
    pub profile: profile::Profile,
    pub extensions: extensions::Extensions,
}

impl Config {
    pub fn set(&mut self, name: &str, value: &str) -> Result<()> {
        let error_message = format!("Unknown setter field: {}", name);

        return if common::is_match(name, core::COMMAND)? {
            Ok(core::set(&mut self.core, &name, &value)?)
        } else if common::is_match(name, portal::COMMAND)? {
            Ok(portal::set(&mut self.portal, &name, &value)?)
        } else if common::is_match(name, extensions::COMMAND)? {
            Ok(extensions::set(&mut self.extensions, &name, &value)?)
        } else if common::is_match(name, profile::COMMAND)? {
            Ok(profile::set(&mut self.profile, &name, &value)?)
        } else {
            Err(format_err!(error_message.clone()))
        };
    }

    pub fn get(&self, name: &str) -> Result<String> {
        let error_message = format!("Unknown getter field: {}", name);

        return if common::is_match(name, core::COMMAND)? {
            Ok(core::get(&self.core, &name)?)
        } else if common::is_match(name, portal::COMMAND)? {
            Ok(portal::get(&self.portal, &name)?)
        } else if common::is_match(name, extensions::COMMAND)? {
            Ok(extensions::get(&self.extensions, &name)?)
        } else if common::is_match(name, profile::COMMAND)? {
            Ok(profile::get(&self.profile, &name)?)
        } else {
            Err(format_err!(error_message.clone()))
        };
    }
}

impl common::FilePath for Config {
    fn file_path() -> Result<std::path::PathBuf> {
        let paths = super::fs::ConfigPaths::new()?;
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
