use anyhow::{format_err, Result};

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Operator {
    GreaterThanEqual,
    GreaterThan,

    LessThanEqual,
    LessThan,

    Equal,
}

impl std::string::ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Self::GreaterThanEqual => ">=",
            Self::GreaterThan => ">",
            Self::LessThanEqual => "<=",
            Self::LessThan => "<",
            Self::Equal => "=",
        }
        .to_string()
    }
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

pub trait Condition {
    fn evaluate(&self, parameters: &super::Parameters) -> Result<bool>;
    fn metadata(&self) -> Box<dyn ConditionMetadata>;
}

pub trait ConditionMetadata: std::fmt::Debug {
    fn name(&self) -> String;
    fn interactive_set_parameter(&self, parameters: &mut super::Parameters) -> Result<()>;
    fn is_parameter_set(&self, parameters: &super::Parameters) -> bool;
}
