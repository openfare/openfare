use anyhow::Result;

pub mod conditions;
mod frequency;
pub mod price;

use super::payees;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentPlan {
    pub id: String,
    pub conditions: std::collections::BTreeMap<conditions::Condition, String>,
    pub payments: Payments,
}

impl PaymentPlan {
    pub fn is_applicable(&self, config: &crate::config::Config) -> Result<bool> {
        let mut all_conditions_pass = true;
        for (condition, value) in &self.conditions {
            all_conditions_pass &= conditions::evaluate(&condition, &value, &config)?;
        }
        Ok(all_conditions_pass)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payments {
    pub total: price::Price,
    pub frequency: frequency::Frequency,
    pub split: Split,
}

pub type Percent = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Split {
    parts: Option<std::collections::BTreeMap<payees::PayeeName, Percent>>,
    remainder: payees::PayeeName,
}
