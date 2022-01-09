use anyhow::{format_err, Result};

pub trait PaymentMethod {
    fn associated_name() -> String
    where
        Self: Sized;
    fn name(&self) -> String;
    fn to_serde_json_value(&self) -> Result<serde_json::Value>;
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PayPal {
    /// PayPal ID.
    id: Option<String>,
    /// Payee email.
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
    fn associated_name() -> String {
        "paypal".to_string()
    }

    fn name(&self) -> String {
        Self::associated_name()
    }

    fn to_serde_json_value(&self) -> Result<serde_json::Value> {
        let s = serde_json::to_string(&self)?;
        Ok(serde_json::from_str(&s)?)
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
    fn associated_name() -> String {
        "btc_lightning_keysend".to_string()
    }

    fn name(&self) -> String {
        Self::associated_name()
    }

    fn to_serde_json_value(&self) -> Result<serde_json::Value> {
        let s = serde_json::to_string(&self)?;
        Ok(serde_json::from_str(&s)?)
    }
}
