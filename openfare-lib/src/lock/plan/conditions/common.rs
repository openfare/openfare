use anyhow::{format_err, Result};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Operator {
    GreaterThanEqual,
    GreaterThan,

    LessThanEqual,
    LessThan,

    Equal,
}

impl std::convert::TryFrom<&str> for Operator {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self> {
        Ok(match value {
            ">=" => Self::GreaterThanEqual,
            ">" => Self::GreaterThan,
            "<=" => Self::LessThanEqual,
            "<" => Self::LessThan,
            "=" => Self::Equal,
            _ => {
                return Err(format_err!("Unknown operator: {}", value));
            }
        })
    }
}

pub fn evaluate_operator<T: std::cmp::PartialOrd>(
    variable_value: &T,
    operator: &Operator,
    condition_value: &T,
) -> bool {
    match operator {
        Operator::GreaterThanEqual => variable_value >= condition_value,
        Operator::GreaterThan => variable_value > condition_value,

        Operator::LessThanEqual => variable_value <= condition_value,
        Operator::LessThan => variable_value < condition_value,

        Operator::Equal => variable_value == condition_value,
    }
}
