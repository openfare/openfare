use anyhow::{format_err, Result};
use std::convert::TryInto;

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

fn get_regex() -> Result<regex::Regex> {
    Ok(regex::Regex::new(r"portal\.(.*)")?)
}

pub fn is_match(name: &str) -> Result<bool> {
    Ok(get_regex()?.is_match(name))
}

pub fn set(portal: &mut Portal, name: &str, value: &str) -> Result<()> {
    let name_error_message = format!("Unknown setting field name: {}", name);

    let captures = get_regex()?
        .captures(name)
        .ok_or(format_err!(name_error_message.clone()))?;
    let field = captures
        .get(1)
        .ok_or(format_err!(name_error_message.clone()))?
        .as_str();

    match field {
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
            return Err(format_err!(name_error_message.clone()));
        }
    }
    Ok(())
}

pub fn get(portal: &Portal, name: &str) -> Result<String> {
    let name_error_message = format!("Unknown setting field name: {}", name);

    let captures = get_regex()?
        .captures(name)
        .ok_or(format_err!(name_error_message.clone()))?;
    let field = captures
        .get(1)
        .ok_or(format_err!(name_error_message.clone()))?
        .as_str();

    Ok(match field {
        "url" => portal.url.to_string(),
        "api-key" => portal.api_key.to_string(),
        "email" => portal.email.clone().unwrap_or_default().to_string(),
        _ => {
            return Err(format_err!(name_error_message.clone()));
        }
    })
}
