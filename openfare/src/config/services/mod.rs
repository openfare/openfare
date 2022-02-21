mod lnpay;
mod portal;

/// Payment services.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Services {
    pub default: String,
    pub portal: portal::Portal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lnpay: Option<lnpay::LnPay>,
}

impl std::default::Default for Services {
    fn default() -> Self {
        Self {
            default: "portal".to_string(),
            portal: portal::Portal::default(),
            lnpay: None,
        }
    }
}

impl std::fmt::Display for Services {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}
