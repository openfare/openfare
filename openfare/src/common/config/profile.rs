use super::common;
use anyhow::{format_err, Result};

pub use openfare_lib::lock::plan::conditions::Parameters;

pub const COMMAND: &'static str = "profile";

pub fn set(parameters: &mut Parameters, name: &str, value: &str) -> Result<()> {
    let error_message = format!("Unknown setter field name: {}", name);
    let field = common::get_field(&name, &COMMAND, &error_message)?;

    match field.as_str() {
        "employees-count" => {
            parameters.employees_count = Some(value.parse::<usize>()?);
            Ok(())
        }
        "commercial" => {
            parameters.commercial = value.parse::<bool>()?;
            Ok(())
        }
        "include-voluntary-plans" => {
            parameters.include_voluntary_plans = value.parse::<bool>()?;
            Ok(())
        }
        _ => Err(format_err!(error_message.clone())),
    }
}

pub fn get(parameters: &Parameters, name: &str) -> Result<String> {
    let error_message = format!("Unknown getter field name: {}", name);
    let field = common::get_field(&name, &COMMAND, &error_message)?;

    Ok(match field.as_str() {
        COMMAND => parameters.to_string(),
        "employees-count" => {
            if let Some(employees_count) = parameters.employees_count {
                employees_count.to_string()
            } else {
                "".to_string()
            }
        }
        "commercial" => parameters.commercial.to_string(),
        "include-voluntary-plans" => parameters.include_voluntary_plans.to_string(),
        _ => return Err(format_err!(error_message.clone())),
    })
}
