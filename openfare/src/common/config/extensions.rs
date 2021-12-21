use crate::common::config::common;
use anyhow::{format_err, Result};

#[derive(
    Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Extensions {
    pub enabled: std::collections::BTreeMap<String, bool>,
    pub registries: std::collections::BTreeMap<String, String>,
}

fn get_regex() -> Result<regex::Regex> {
    Ok(regex::Regex::new(r"extensions\.enabled\.(.*)")?)
}

pub fn is_match(name: &str) -> Result<bool> {
    Ok(get_regex()?.is_match(name))
}

pub fn set(extensions: &mut Extensions, name: &str, value: &str) -> Result<()> {
    let name_error_message = format!("Unknown setting field name: {}", name);

    let captures = get_regex()?
        .captures(name)
        .ok_or(format_err!(name_error_message.clone()))?;
    let extension_name = captures
        .get(1)
        .ok_or(format_err!(name_error_message.clone()))?
        .as_str();

    let value = common::bool_from_string(value)?;

    if !extensions.enabled.contains_key(extension_name) {
        return Err(format_err!(name_error_message.clone()));
    }
    extensions.enabled.insert(extension_name.to_string(), value);

    Ok(())
}

pub fn get(extensions: &Extensions, name: &str) -> Result<String> {
    let name_error_message = format!("Unknown setting field name: {}", name);

    let captures = get_regex()?
        .captures(name)
        .ok_or(format_err!(name_error_message.clone()))?;
    let extension_name = captures
        .get(1)
        .ok_or(format_err!(name_error_message.clone()))?
        .as_str();

    Ok(extensions
        .enabled
        .get(extension_name)
        .ok_or(format_err!(name_error_message.clone()))?
        .to_string())
}
