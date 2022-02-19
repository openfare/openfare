use anyhow::{format_err, Result};
use std::str::FromStr;

pub type Name = String;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum PaymentMethods {
    #[serde(rename = "paypal")]
    PayPal,
    #[serde(rename = "btc_lightning_keysend")]
    BtcLightningKeysend,
}

impl std::str::FromStr for PaymentMethods {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "paypal" => Self::PayPal,
            // TODO: _ --> -
            "btc_lightning_keysend" => Self::BtcLightningKeysend,
            _ => {
                return Err(anyhow::format_err!("Unknown payment method: {}", s));
            }
        })
    }
}

impl std::fmt::Display for PaymentMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::PayPal => "paypal",
            Self::BtcLightningKeysend => "btc_lightning_keysend",
        };
        write!(f, "{}", s)
    }
}

pub trait PaymentMethod {
    // TODO: remove associated_name method and name method?
    fn method_type(&self) -> PaymentMethods;
    fn to_serde_json_value(&self) -> Result<serde_json::Value>;
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PayPal {
    /// PayPal ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    /// Payee email.
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

impl PayPal {
    pub fn new(id: &Option<String>, email: &Option<String>) -> Result<Self> {
        if id.is_none() && email.is_none() {
            return Err(format_err!("Both id and email fields can not be none."));
        }
        Ok(Self {
            id: id.clone(),
            email: email.clone(),
        })
    }
}

impl PaymentMethod for PayPal {
    fn method_type(&self) -> PaymentMethods {
        PaymentMethods::PayPal
    }

    fn to_serde_json_value(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(&self)?)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BtcLightningKeysend {
    public_key: String,
}

impl BtcLightningKeysend {
    pub fn new(public_key: &str) -> Result<Self> {
        Ok(Self {
            public_key: public_key.to_string(),
        })
    }
}

impl PaymentMethod for BtcLightningKeysend {
    fn method_type(&self) -> PaymentMethods {
        PaymentMethods::BtcLightningKeysend
    }

    fn to_serde_json_value(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(&self)?)
    }
}

pub fn check(
    payment_methods: &std::collections::BTreeMap<PaymentMethods, serde_json::Value>,
) -> Result<()> {
    for (method, json_value) in payment_methods {
        let clean_json_value = match method {
            PaymentMethods::PayPal => {
                let method = serde_json::from_value::<PayPal>(json_value.clone())?;
                serde_json::to_value(&method)?
            }
            PaymentMethods::BtcLightningKeysend => {
                let method = serde_json::from_value::<BtcLightningKeysend>(json_value.clone())?;
                serde_json::to_value(&method)?
            }
        };

        if json_value != &clean_json_value {
            return Err(anyhow::format_err!(
                "Found unexpected field(s): {}",
                json_value
            ));
        }
    }
    Ok(())
}
