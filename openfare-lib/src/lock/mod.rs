pub mod payee;
pub mod plan;

pub static FILE_NAME: &str = "OPENFARE.lock";
pub static SCHEME_VERSION: &str = "1";

/// A software package's OpenFare lock file (OPENFARE.lock).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Lock {
    #[serde(rename = "scheme-version")]
    pub scheme_version: String,
    pub plans: plan::Plans,
    pub payees: payee::Payees,
}

impl std::default::Default for Lock {
    fn default() -> Self {
        Self {
            scheme_version: SCHEME_VERSION.to_string(),
            plans: plan::Plans::new(),
            payees: payee::Payees::new(),
        }
    }
}
