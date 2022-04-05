use super::common;
use anyhow::{format_err, Result};
use lazy_regex::regex;
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EmployeesCount {
    operator: common::Operator,
    count: usize,
}

impl std::convert::TryFrom<&str> for EmployeesCount {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (operator, count) = parse_value(&value)?;
        Ok(Self { operator, count })
    }
}

impl common::Condition for EmployeesCount {
    fn evaluate(&self, parameters: &crate::lock::plan::conditions::Parameters) -> Result<bool> {
        let employees_count = parameters.employees_count.ok_or(format_err!(
            "Attempting to evaluate plan conditions using unset parameter `{}`.",
            self.metadata().name()
        ))?;
        let result =
            common::evaluate_operator::<usize>(&employees_count, &self.operator, &self.count);
        Ok(result)
    }

    fn metadata(&self) -> Box<dyn common::ConditionMetadata> {
        Box::new(EmployeesCountMetadata) as Box<dyn common::ConditionMetadata>
    }
}

impl serde::Serialize for EmployeesCount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{} {}", self.operator.to_string(), self.count,).as_str())
    }
}

struct Visitor {
    marker: std::marker::PhantomData<fn() -> EmployeesCount>,
}

impl Visitor {
    fn new() -> Self {
        Visitor {
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = EmployeesCount;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string such as '> 100'")
    }

    fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let (operator, count) = parse_value(&value).expect("parse employee-count condition value");
        Ok(Self::Value { operator, count })
    }
}

impl<'de> serde::Deserialize<'de> for EmployeesCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::new())
    }
}

#[derive(Debug, Clone)]
struct EmployeesCountMetadata;

impl common::ConditionMetadata for EmployeesCountMetadata {
    fn name(&self) -> String {
        "employees-count".to_string()
    }

    fn description(&self) -> String {
        "Employees count.".to_string()
    }

    fn interactive_set_parameter(
        &self,
        parameters: &mut crate::lock::plan::conditions::Parameters,
    ) -> Result<()> {
        println!("Select a range for the number of employees (x) within your organization:");
        let items = vec![
            "1 <= x < 50",
            "50 <= x < 500",
            "500 <= x < 1000",
            "1000 <= x",
        ];
        let chosen: Vec<usize> = dialoguer::MultiSelect::new().items(&items).interact()?;

        // if let Some(first_index) = chosen.first() {
        //     if let Some(value) = items.get(*first_index) {
        //         let (_operator, count) = parse_value(&value)?;
        //         parameters.employees_count = Some(count);
        //     }
        // }
        Ok(())
    }

    fn is_parameter_set(&self, parameters: &crate::lock::plan::conditions::Parameters) -> bool {
        parameters.employees_count.is_some()
    }

    fn validate_parameter(&self, value: &str) -> Result<()> {
        let (operator, count) = parse_value(&value)?;
        if operator == common::Operator::Equal {
            if count == 0 {
                return Err(format_err!("Invalid value: {}", value));
            }
        }
        Ok(())
    }
}

fn parse_value(value: &str) -> Result<(common::Operator, usize)> {
    let re = regex!(r"(?P<operator>(>=)|(<=)|(<)|(>)|(=))\s*(?P<quantity>[0-9]+)");
    let captures = re
        .captures(value)
        .ok_or(format_err!("Regex failed to capture field."))?;

    let operator_match = captures
        .name("operator")
        .expect("extract operator from regex capture")
        .as_str();
    let operator = common::Operator::try_from(operator_match)?;

    let quantity_match = captures
        .name("quantity")
        .expect("extract quantity from regex capture")
        .as_str();
    let quantity = quantity_match.parse::<usize>()?;

    Ok((operator, quantity))
}

#[test]
fn test_from_str() -> Result<()> {
    let mut parameters = crate::lock::plan::conditions::Parameters::default();
    parameters.employees_count = Some(99);

    let employees_count = EmployeesCount::try_from("<= 99")?;
    assert!(employees_count.evaluate(&parameters)?);
    Ok(())
}
