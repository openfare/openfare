use anyhow::{format_err, Result};

pub use openfare_lib::lock::plan::conditions::Parameters;

fn get_regex() -> Result<regex::Regex> {
    Ok(regex::Regex::new(r"metrics\.(.*)")?)
}

pub fn is_match(name: &str) -> Result<bool> {
    Ok(get_regex()?.is_match(name))
}

pub fn set(parameters: &mut Parameters, name: &str, value: &str) -> Result<()> {
    let name_error_message = format!("Unknown setting field name: {}", name);

    let captures = get_regex()?
        .captures(name)
        .ok_or(format_err!(name_error_message.clone()))?;
    let field = captures
        .get(1)
        .ok_or(format_err!(name_error_message.clone()))?
        .as_str();

    match field {
        "employees-count" => {
            parameters.employees_count = Some(value.parse::<usize>()?);
            Ok(())
        }
        _ => Err(format_err!(name_error_message.clone())),
    }
}

pub fn get(parameters: &Parameters, name: &str) -> Result<String> {
    let name_error_message = format!("Unknown setting field name: {}", name);

    let captures = get_regex()?
        .captures(name)
        .ok_or(format_err!(name_error_message.clone()))?;
    let field = captures
        .get(1)
        .ok_or(format_err!(name_error_message.clone()))?
        .as_str();

    match field {
        "employees-count" => Ok(if let Some(employees_count) = parameters.employees_count {
            employees_count.to_string()
        } else {
            "".to_string()
        }),
        _ => Err(format_err!(name_error_message.clone())),
    }
}
