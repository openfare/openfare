use super::common;
use anyhow::{format_err, Result};

pub const COMMAND: &'static str = "profile";

#[derive(
    Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Profile {
    pub url: Option<String>,
    #[serde(flatten)]
    pub parameters: openfare_lib::lock::plan::conditions::Parameters,
}

pub fn set(profile: &mut Profile, name: &str, value: &str) -> Result<()> {
    let error_message = format!("Unknown setter field name: {}", name);
    let field = common::get_field(&name, &COMMAND, &error_message)?;

    match field.as_str() {
        "employees-count" => {
            profile.parameters.employees_count = Some(value.parse::<usize>()?);
        }
        "commercial" => {
            profile.parameters.commercial = value.parse::<bool>()?;
        }
        "include-voluntary-plans" => {
            profile.parameters.include_voluntary_plans = value.parse::<bool>()?;
        }
        "url" => {
            let url = value.parse::<String>()?;
            profile.url = if url != "".to_string() {
                Some(value.parse::<String>()?)
            } else {
                None
            };
        }
        _ => {
            return Err(format_err!(error_message.clone()));
        }
    };
    Ok(())
}

pub fn get(profile: &Profile, name: &str) -> Result<String> {
    let error_message = format!("Unknown getter field name: {}", name);
    let field = common::get_field(&name, &COMMAND, &error_message)?;

    Ok(match field.as_str() {
        COMMAND => profile.to_string(),
        "employees-count" => {
            if let Some(employees_count) = profile.parameters.employees_count {
                employees_count.to_string()
            } else {
                "".to_string()
            }
        }
        "commercial" => profile.parameters.commercial.to_string(),
        "include-voluntary-plans" => profile.parameters.include_voluntary_plans.to_string(),
        "url" => profile.url.clone().unwrap_or("".to_string()),
        _ => return Err(format_err!(error_message.clone())),
    })
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}
