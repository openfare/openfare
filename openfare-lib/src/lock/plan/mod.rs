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

// TODO: Rename to 'Plan'.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentPlan {
    pub r#type: PlanType,
    pub conditions: conditions::Conditions,
    pub payments: Payments,
}

impl PaymentPlan {
    pub fn is_applicable(
        &self,
        parameters: &crate::lock::plan::conditions::Parameters,
    ) -> Result<bool> {
        Ok(match self.r#type {
            PlanType::Voluntary => {
                // Voluntary plans are subject to conditions.
                parameters.include_voluntary_plans && self.conditions.evaluate(&parameters)?
            }
            PlanType::Compulsory => {
                // Non-commercial not subject to compulsory plans.
                if !parameters.commercial {
                    false
                } else {
                    // Commercial are subject to compulsory plans based on conditions.
                    self.conditions.evaluate(&parameters)?
                }
            }
        })
    }
}

/// Filter for applicable plans.
pub fn filter_applicable(
    plans: &Plans,
    parameters: &crate::lock::plan::conditions::Parameters,
) -> Result<Plans> {
    let mut applicable_plans = Plans::new();
    for (plan_id, plan) in plans {
        if plan.is_applicable(&parameters)? {
            applicable_plans.insert(plan_id.clone(), plan.clone());
        }
    }
    Ok(applicable_plans)
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<price::Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<std::collections::BTreeMap<payee::Name, u64>>,
}
