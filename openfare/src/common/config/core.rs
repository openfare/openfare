use super::common;
use anyhow::{format_err, Result};

pub const COMMAND: &'static str = "core";

#[derive(
    Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Core {
    #[serde(rename = "preferred-currency")]
    pub preferred_currency: openfare_lib::lock::plan::price::Currency,
}

impl std::fmt::Display for Core {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}

pub fn set(_core: &mut Core, name: &str, _value: &str) -> Result<()> {
    let error_message = format!("Unknown setter field name: {}", name);
    let field = common::get_field(&name, &COMMAND, &error_message)?;

    match field.as_str() {
        "preferred-currency" => {
            // TODO: support alternative preferred currency
            // core.preferred_currency = value.try_into()?;
            return Err(format_err!(
                "'preferred-currency' is not currently modifiable."
            ));
        }
        _ => {
            return Err(format_err!(error_message.clone()));
        }
    }
}

pub fn get(core: &Core, name: &str) -> Result<String> {
    let error_message = format!("Unknown getter field name: {}", name);
    let field = common::get_field(&name, &COMMAND, &error_message)?;

    Ok(match field.as_str() {
        COMMAND => core.to_string(),
        "preferred-currency" => core.preferred_currency.to_string(),
        _ => {
            return Err(format_err!(error_message.clone()));
        }
    })
}
