#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Portal {
    pub url: url::Url,
    #[serde(rename = "api-key")]
    pub api_key: openfare_lib::api::services::portal::ApiKey,
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
