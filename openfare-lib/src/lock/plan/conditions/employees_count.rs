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

impl EmployeesCount {
    pub fn evaluate(&self, config: &crate::config::Config) -> Result<bool> {
        let employees_count = config.employees_count.ok_or(format_err!(
            "Attempting to evaluate condition but the `employees-count` metric is unset."
        ))?;
        let result =
            common::evaluate_operator::<usize>(&employees_count, &self.operator, &self.count);
        Ok(result)
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
    let mut config = crate::config::Config::default();
    config.employees_count = Some(99);

    let employees_count = EmployeesCount::try_from("<= 99")?;
    assert!(employees_count.evaluate(&config)?);
    Ok(())
}
