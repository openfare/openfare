mod config;
mod lnpay;
mod portal;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ServiceType {
    Portal,
    #[serde(rename = "lnpay")]
    LnPay,
}

pub fn pay(
    package_locks: &std::collections::BTreeMap<
        openfare_lib::package::Package,
        openfare_lib::lock::Lock,
    >,
    // service_type: 
    config: &crate::config::Config,
) {
    // config.services.default
}

/// Payment services config.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub default: ServiceType,
    pub portal: portal::Portal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lnpay: Option<lnpay::LnPay>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            default: ServiceType::Portal,
            portal: portal::Portal::default(),
            lnpay: None,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}