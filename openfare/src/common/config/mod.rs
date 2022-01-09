use anyhow::{format_err, Result};

mod common;
mod core;
mod extensions;
mod metrics;
mod payees;

pub use common::FileStore;
pub use metrics::Metrics;
pub use payees::Payees;

#[derive(
    Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Config {
    pub core: core::Core,
    pub metrics: metrics::Metrics,
    pub extensions: extensions::Extensions,
}

impl Config {
    pub fn set(&mut self, name: &str, value: &str) -> Result<()> {
        let name_error_message = format!("Unknown settings field: {}", name);

        return if core::is_match(name)? {
            Ok(core::set(&mut self.core, &name, &value)?)
        } else if extensions::is_match(name)? {
            Ok(extensions::set(&mut self.extensions, &name, &value)?)
        } else if metrics::is_match(name)? {
            Ok(metrics::set(&mut self.metrics, &name, &value)?)
        } else {
            Err(format_err!(name_error_message.clone()))
        };
    }

    pub fn get(&self, name: &str) -> Result<String> {
        let name_error_message = format!("Unknown settings field: {}", name);

        return if core::is_match(name)? {
            Ok(core::get(&self.core, &name)?)
        } else if extensions::is_match(name)? {
            Ok(extensions::get(&self.extensions, &name)?)
        } else if metrics::is_match(name)? {
            Ok(metrics::get(&self.metrics, &name)?)
        } else {
            Err(format_err!(name_error_message.clone()))
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
            serde_json::to_string(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}
