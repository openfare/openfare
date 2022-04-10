use anyhow::Result;

mod common;
mod employees_count;
mod expiration;
mod for_profit;
pub mod parameters;

pub use common::{Condition, ConditionMetadata};
pub use employees_count::EmployeesCount;
pub use expiration::Expiration;
pub use for_profit::ForProfit;
pub use parameters::Parameters;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Conditions {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub for_profit: Option<ForProfit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<Expiration>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "employees-count")]
    pub employees_count: Option<EmployeesCount>,
}

impl Conditions {
    pub fn as_vec(&self) -> Vec<Box<dyn Condition>> {
        let mut vec = Vec::new();
        if let Some(for_profit) = &self.for_profit {
            vec.push(Box::new(for_profit.clone()) as Box<dyn Condition>);
        }
        if let Some(expiration) = &self.expiration {
            vec.push(Box::new(expiration.clone()) as Box<dyn Condition>);
        }
        if let Some(employees_count) = &self.employees_count {
            vec.push(Box::new(employees_count.clone()) as Box<dyn Condition>);
        }
        vec
    }

    pub fn metadata(&self) -> Vec<Box<dyn ConditionMetadata>> {
        self.as_vec()
            .iter()
            .map(|condition| condition.metadata())
            .collect()
    }

    pub fn evaluate(&self, parameters: &crate::lock::plan::conditions::Parameters) -> Result<bool> {
        let all_conditions_pass = self
            .as_vec()
            .iter()
            .all(|condition| condition.evaluate(parameters).unwrap_or(false));
        Ok(all_conditions_pass)
    }

    pub fn set_some(&mut self, incoming: &Self) {
        if self.for_profit.is_none() {
            self.for_profit = incoming.for_profit.clone();
        }
        if self.expiration.is_none() {
            self.expiration = incoming.expiration.clone();
        }
        if self.employees_count.is_none() {
            self.employees_count = incoming.employees_count.clone();
        }
    }
}
