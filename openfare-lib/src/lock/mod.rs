pub mod payee;
pub mod plan;

/// A software package's OpenFare lock file (OPENFARE.lock).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Lock {
    pub plans: Vec<plan::PaymentPlan>,
    pub payees: std::collections::BTreeMap<payee::PayeeName, payee::Payee>,
}
