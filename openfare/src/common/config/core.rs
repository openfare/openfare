use anyhow::{format_err, Result};
use std::convert::TryInto;

#[derive(
    Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Core {
    #[serde(rename = "preferred-currency")]
    pub preferred_currency: openfare_lib::lock::plan::price::Currency,
}

fn get_regex() -> Result<regex::Regex> {
    Ok(regex::Regex::new(r"core\.(.*)")?)
}

pub fn is_match(name: &str) -> Result<bool> {
    Ok(get_regex()?.is_match(name))
}

pub fn set(core: &mut Core, name: &str, value: &str) -> Result<()> {
    let name_error_message = format!("Unknown setting field name: {}", name);

    let captures = get_regex()?
        .captures(name)
        .ok_or(format_err!(name_error_message.clone()))?;
    let field = captures
        .get(1)
        .ok_or(format_err!(name_error_message.clone()))?
        .as_str();

    match field {
        "preferred-currency" => {
            core.preferred_currency = value.try_into()?;
        }
        _ => {
            return Err(format_err!(name_error_message.clone()));
        }
    }
    Ok(())
}

pub fn get(core: &Core, name: &str) -> Result<String> {
    let name_error_message = format!("Unknown setting field name: {}", name);

    let captures = get_regex()?
        .captures(name)
        .ok_or(format_err!(name_error_message.clone()))?;
    let field = captures
        .get(1)
        .ok_or(format_err!(name_error_message.clone()))?
        .as_str();

    Ok(match field {
        "preferred-currency" => core.preferred_currency.to_string(),
        _ => {
            return Err(format_err!(name_error_message.clone()));
        }
    })
}
