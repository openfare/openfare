use anyhow::Result;

mod common;
mod current_time;
mod employees_count;

pub use current_time::CurrentTime;
pub use employees_count::EmployeesCount;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Conditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<CurrentTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub employees_count: Option<EmployeesCount>,
}

impl Conditions {
    pub fn evaluate(&self, parameters: &crate::lock::plan::conditions::Parameters) -> Result<bool> {
        let mut all_conditions_pass = true;
        if let Some(current_time) = &self.current_time {
            all_conditions_pass &= current_time.evaluate()?;
        }
        if let Some(employees_count) = &self.employees_count {
            all_conditions_pass &= employees_count.evaluate(&parameters)?;
        }
        Ok(all_conditions_pass)
    }

    pub fn set_some(&mut self, incoming: &Self) {
        if self.current_time.is_none() {
            self.current_time = incoming.current_time.clone();
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
    pub commercial: bool,
    #[serde(rename = "include-voluntary-donations")]
    pub include_voluntary_plans: bool,
}

impl std::default::Default for Parameters {
    fn default() -> Self {
        Self {
            employees_count: None,
            commercial: true,
            include_voluntary_plans: true,
        }
    }
}
