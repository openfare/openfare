use anyhow::Result;

pub mod payee;
pub mod plan;
mod schema;
pub mod shares;

#[cfg(test)]
mod tests;

pub static FILE_NAME: &str = "OpenFare.lock";

/// A software package's OpenFare lock file (OpenFare.lock).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lock {
    #[serde(rename = "scheme-version")]
    pub scheme_version: String,
    pub plans: plan::Plans,
    pub payees: payee::Payees,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<shares::Shares>,
}

impl Lock {
    /// Validate lock.
    pub fn validate(&self) -> Result<()> {
        let value = serde_json::to_value(&self)?;
        schema::validate(&value)
    }
}

impl std::default::Default for Lock {
    fn default() -> Self {
        Self {
            scheme_version: schema::VERSION.to_string(),
            plans: plan::Plans::new(),
            payees: payee::Payees::new(),
            shares: None,
        }
    }
}
