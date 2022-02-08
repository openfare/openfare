use crate::common::config::common;
use anyhow::{format_err, Result};

pub const COMMAND: &'static str = "extensions";

#[derive(
    Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Extensions {
    pub enabled: std::collections::BTreeMap<String, bool>,
    pub registries: std::collections::BTreeMap<String, String>,
}

impl std::fmt::Display for Extensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}

pub fn set(extensions: &mut Extensions, name: &str, value: &str) -> Result<()> {
    let error_message = format!("Unknown setter field name: {}", name);
    let extension_name = common::get_field(&name, &COMMAND, &error_message)?;

    let value = common::bool_from_string(value)?;

    if !extensions.enabled.contains_key(&extension_name) {
        return Err(format_err!(error_message.clone()));
    }
    extensions.enabled.insert(extension_name.to_string(), value);

    Ok(())
}

pub fn get(extensions: &Extensions, name: &str) -> Result<String> {
    let error_message = format!("Unknown getter field name: {}", name);
    let field = common::get_field(&name, &COMMAND, &error_message)?;

    Ok(match field.as_str() {
        COMMAND => extensions.to_string(),
        extension_name => extensions
            .enabled
            .get(extension_name)
            .ok_or(format_err!(error_message.clone()))?
            .to_string(),
    })
}
