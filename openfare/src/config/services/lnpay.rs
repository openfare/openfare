#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct LnPay {
    #[serde(rename = "api-key")]
    pub api_key: String,
}

impl std::fmt::Display for LnPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}
