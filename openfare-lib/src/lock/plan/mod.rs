use anyhow::{format_err, Result};

pub mod conditions;
pub mod price;

use super::payee;

pub type Id = String;
pub type Plans = std::collections::BTreeMap<Id, PaymentPlan>;
pub type SplitPercent = String;

pub fn next_id(plans: &Plans) -> Result<Id> {
    let mut ids = plans.iter().map(|(id, _plan)| id).collect::<Vec<_>>();
    ids.sort();
    for (count, id) in ids.iter().enumerate() {
        let id = id.parse::<usize>().expect("parse plan ID as number");
        if count < id {
            return Ok(count.to_string());
        }
    }
    Ok(ids.len().to_string())
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanType {
    Compulsory,
    Voluntary,
}

impl std::str::FromStr for PlanType {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> std::result::Result<Self, anyhow::Error> {
        Ok(match value {
            "compulsory" => PlanType::Compulsory,
            "voluntary" => PlanType::Voluntary,
            _ => {
                return Err(format_err!(
                    "Unsupported plan type: {}. Supported values: [compulsory|voluntary].",
                    value
                ));
            }
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentPlan {
    pub r#type: PlanType,
    pub conditions: conditions::Conditions,
    pub payments: Payments,
}

impl PaymentPlan {
    pub fn is_applicable(&self, config: &crate::config::Config) -> Result<bool> {
        Ok(self.conditions.evaluate(&config)?)
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<price::Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<std::collections::BTreeMap<payee::Name, u64>>,
}
