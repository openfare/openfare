use super::common;
use anyhow::{format_err, Result};
use std::convert::TryInto;

pub const COMMAND: &'static str = "portal";

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Portal {
    pub url: url::Url,
    #[serde(rename = "api-key")]
    pub api_key: openfare_lib::api::portal::common::ApiKey,
    pub email: Option<String>,
}

impl std::default::Default for Portal {
    fn default() -> Self {
        let api_key = {
            let uuid = uuid::Uuid::new_v4();
            let mut encode_buffer = uuid::Uuid::encode_buffer();
            let uuid = uuid.to_hyphenated().encode_lower(&mut encode_buffer);
            uuid.to_string()
        };
        Self {
            url: url::Url::parse("https://openfare.dev/").unwrap(),
            api_key,
            email: None,
        }
    }
}

impl std::fmt::Display for Portal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}

pub fn set(portal: &mut Portal, name: &str, value: &str) -> Result<()> {
    let error_message = format!("Unknown setting field name: {}", name);
    let field = common::get_field(&name, &COMMAND, &error_message)?;

    match field.as_str() {
        "url" => {
            portal.url = value.try_into()?;
        }
        "api-key" => {
            portal.api_key = value.try_into()?;
        }
        "email" => {
            portal.email = Some(value.try_into()?);
        }
        _ => {
            return Err(format_err!(error_message.clone()));
        }
    }
    Ok(())
}

pub fn get(portal: &Portal, name: &str) -> Result<String> {
    let error_message = format!("Unknown getter field name: {}", name);
    let field = common::get_field(&name, &COMMAND, &error_message)?;

    Ok(match field.as_str() {
        "url" => portal.url.to_string(),
        "api-key" => portal.api_key.to_string(),
        "email" => portal.email.clone().unwrap_or_default().to_string(),
        COMMAND => portal.to_string(),
        _ => {
            return Err(format_err!(error_message.clone()));
        }
    })
}
