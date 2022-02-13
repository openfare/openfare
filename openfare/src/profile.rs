use crate::common;
use anyhow::Result;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Profile(openfare_lib::lock::payee::Payee);

impl std::ops::Deref for Profile {
    type Target = openfare_lib::lock::payee::Payee;

    fn deref(&self) -> &openfare_lib::lock::payee::Payee {
        &self.0
    }
}

impl std::ops::DerefMut for Profile {
    fn deref_mut(&mut self) -> &mut openfare_lib::lock::payee::Payee {
        &mut self.0
    }
}

impl common::config::FilePath for Profile {
    fn file_path() -> Result<std::path::PathBuf> {
        let paths = common::fs::ConfigPaths::new()?;
        Ok(paths.profile_file)
    }
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}
