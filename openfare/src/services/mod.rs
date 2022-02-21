use anyhow::Result;

pub mod common;
mod lnpay;
mod portal;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Service {
    Portal,
    #[serde(rename = "lnpay")]
    LnPay,
}

impl std::str::FromStr for Service {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "portal" => Self::Portal,
            "lnpay" => Self::LnPay,
            _ => {
                return Err(anyhow::format_err!("Unknown payment service: {}", s));
            }
        })
    }
}

pub fn pay(
    all_extension_locks: &Vec<common::ExtensionLocks>,
    service: &Option<Service>,
    config: &crate::config::Config,
) -> Result<()> {
    let service = service.clone().unwrap_or(config.services.default.clone());
    match service {
        Service::Portal => portal::pay(&all_extension_locks, &config)?,
        Service::LnPay => lnpay::pay(&all_extension_locks, &config)?,
    }
    Ok(())
}
