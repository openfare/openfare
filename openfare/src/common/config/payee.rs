use anyhow::Result;

use super::super::fs;
use super::common;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Payee(openfare_lib::lock::payee::Payee);

impl std::ops::Deref for Payee {
    type Target = openfare_lib::lock::payee::Payee;

    fn deref(&self) -> &openfare_lib::lock::payee::Payee {
        &self.0
    }
}

impl std::ops::DerefMut for Payee {
    fn deref_mut(&mut self) -> &mut openfare_lib::lock::payee::Payee {
        &mut self.0
    }
}

impl common::FilePath for Payee {
    fn file_path() -> Result<std::path::PathBuf> {
        let paths = fs::ConfigPaths::new()?;
        Ok(paths.payee_file)
    }
}

impl std::fmt::Display for Payee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}
