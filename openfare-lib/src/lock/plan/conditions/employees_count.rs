use super::common;
use anyhow::Result;

use strum::IntoEnumIterator;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EmployeesCount(Range);

#[derive(
    Debug,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    strum::EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(into = "String", try_from = "String")]
pub enum Range {
    GreaterEqual1To50,
    GreaterEqual50To150,
    GreaterEqual150To500,
    GreaterEqual500To1000,
    GreaterEqual1000,
}

impl Range {
    pub fn evaluate(&self, employees_count: &Self) -> bool {
        self == employees_count
    }
}

impl std::string::ToString for Range {
    fn to_string(&self) -> String {
        match self {
            Self::GreaterEqual1To50 => "1 <= count < 50",
            Self::GreaterEqual50To150 => "50 <= count < 150",
            Self::GreaterEqual150To500 => "150 <= count < 500",
            Self::GreaterEqual500To1000 => "500 <= count < 1000",
            Self::GreaterEqual1000 => "1000 <= count",
        }
        .to_string()
    }
}

impl Into<String> for Range {
    fn into(self) -> String {
        self.to_string()
    }
}

impl std::convert::TryFrom<&str> for Range {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for range in Self::iter() {
            if range.to_string().as_str() == value {
                return Ok(range);
            }
        }
        let error_message = format!(
            "Error parsing employees count range: {}\nAccepted values:\n{}",
            value,
            Self::iter()
                .map(|range| range.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
        Err(anyhow::format_err!(error_message))
    }
}

impl std::convert::TryFrom<String> for Range {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        for range in Self::iter() {
            if range.to_string().as_str() == value {
                return Ok(range);
            }
        }

        let error_message = format!(
            "Error parsing employees count range: {}\nAccepted values:\n{}",
            value,
            Self::iter()
                .map(|range| range.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
        Err(anyhow::format_err!(error_message))
    }
}

impl std::convert::TryFrom<&str> for EmployeesCount {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(Range::try_from(value)?))
    }
}

impl common::Condition for EmployeesCount {
    fn evaluate(&self, parameters: &crate::lock::plan::conditions::Parameters) -> Result<bool> {
        let employees_count = parameters
            .employees_count
            .as_ref()
            .ok_or(anyhow::format_err!(
                "Attempting to evaluate plan conditions using unset parameter `{}`.",
                self.metadata().name()
            ))?;
        Ok(self.0.evaluate(&employees_count))
    }

    fn metadata(&self) -> Box<dyn common::ConditionMetadata> {
        Box::new(EmployeesCountMetadata) as Box<dyn common::ConditionMetadata>
    }
}

#[derive(Debug, Clone)]
struct EmployeesCountMetadata;

impl common::ConditionMetadata for EmployeesCountMetadata {
    fn name(&self) -> String {
        "employees-count".to_string()
    }

    fn interactive_set_parameter(
        &self,
        parameters: &mut crate::lock::plan::conditions::Parameters,
    ) -> Result<()> {
        println!("Select a range for the number of employees (count) within your organization:");
        let ranges = Range::iter().collect::<Vec<_>>();
        let items = ranges.iter().map(|r| r.to_string()).collect::<Vec<_>>();
        let index = dialoguer::Select::new().items(&items).interact()?;

        if let Some(range) = ranges.get(index) {
            parameters.employees_count = Some(range.clone());
        }
        Ok(())
    }

    fn is_parameter_set(&self, parameters: &crate::lock::plan::conditions::Parameters) -> bool {
        parameters.employees_count.is_some()
    }
}

#[test]
fn test_from_str() -> Result<()> {
    use common::Condition;

    let range = Range::GreaterEqual1To50;

    let mut parameters = crate::lock::plan::conditions::Parameters::default();
    parameters.employees_count = Some(range.clone());

    let employees_count = EmployeesCount::try_from(range.to_string().as_str())?;
    assert!(employees_count.evaluate(&parameters)?);
    Ok(())
}
