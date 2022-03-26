use anyhow::Result;

mod common;
mod employees_count;
mod expiration;
mod for_profit;

pub use employees_count::EmployeesCount;
pub use expiration::Expiration;
pub use for_profit::ForProfit;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Conditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_profit: Option<ForProfit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<Expiration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub employees_count: Option<EmployeesCount>,
}

impl Conditions {
    pub fn evaluate(&self, parameters: &crate::lock::plan::conditions::Parameters) -> Result<bool> {
        let mut all_conditions_pass = true;
        if let Some(for_profit) = &self.for_profit {
            all_conditions_pass &= for_profit.evaluate(&parameters)?;
        }
        if let Some(expiration) = &self.expiration {
            all_conditions_pass &= expiration.evaluate()?;
        }
        if let Some(employees_count) = &self.employees_count {
            all_conditions_pass &= employees_count.evaluate(&parameters)?;
        }
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Parameters {
    #[serde(rename = "employees-count")]
    pub employees_count: Option<usize>,

    #[serde(rename = "for-profit")]
    pub for_profit: Option<bool>,

    pub commercial: bool,

    #[serde(rename = "include-voluntary-donations")]
    pub include_voluntary_plans: bool,
}

impl std::default::Default for Parameters {
    fn default() -> Self {
        Self {
            employees_count: None,
            for_profit: None,
            commercial: true,
            include_voluntary_plans: true,
        }
    }
}

impl std::fmt::Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}
