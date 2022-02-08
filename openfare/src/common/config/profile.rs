use anyhow::{format_err, Result};

pub use openfare_lib::lock::plan::conditions::Parameters;

fn get_regex() -> Result<regex::Regex> {
    Ok(regex::Regex::new(r"profile(\.(.*))?")?)
}

pub fn is_match(name: &str) -> Result<bool> {
    Ok(get_regex()?.is_match(name))
}

fn get_field(name_arg: &str, error_message: &str) -> Result<String> {
    let captures = get_regex()?
        .captures(name_arg)
        .ok_or(format_err!(error_message.to_string()))?;

    let field = if let Some(field) = captures.get(2) {
        field.as_str().to_string()
    } else if let Some(field) = captures.get(0) {
        field.as_str().to_string()
    } else {
        return Err(format_err!(error_message.to_string()));
    };
    Ok(field)
}

pub fn set(parameters: &mut Parameters, name: &str, value: &str) -> Result<()> {
    let error_message = format!("Unknown setting field name: {}", name);
    let field = get_field(&name, &error_message)?;

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
    let error_message = format!("Unknown getting field name: {}", name);
    let field = get_field(&name, &error_message)?;

    Ok(match field.as_str() {
        "profile" => parameters.to_string(),
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
