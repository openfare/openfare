use anyhow::Result;

pub mod lnpay;
mod portal;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Service {
    #[serde(rename = "portal")]
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
    donation: &Option<openfare_lib::price::Price>,
    items: &Vec<openfare_lib::api::services::basket::Item>,
    service: &Option<Service>,
    config: &crate::config::Config,
) -> Result<()> {
    println!("Found {} packages with OpenFare support.", items.len());
    if items.is_empty() {
        return Ok(());
    }
    let service = service.clone().unwrap_or(config.services.default.clone());
    match service {
        Service::Portal => portal::pay(&items, &config)?,
        Service::LnPay => {
            let donation_splits = if let Some(donation) = donation {
                Some(crate::payments::donation_splits(
                    &donation,
                    &items,
                    lnpay::is_payee_applicable,
                )?)
            } else {
                None
            };
            lnpay::pay(&donation_splits, &items, &config)?
        }
    }
    Ok(())
}

pub fn lnurl_receive_address(
    service: &Service,
    config: &crate::config::Config,
) -> Result<Option<String>> {
    Ok(match service {
        Service::Portal => None,
        Service::LnPay => Some(lnpay::lnurl_receive_address(&config)?),
    })
}
