use anyhow::{format_err, Result};

pub type Name = String;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum Methods {
    #[serde(rename = "paypal")]
    PayPal,
    #[serde(rename = "btc-lightning")]
    BtcLightning,
}

pub trait MethodType {
    fn type_method() -> Methods;
}

pub trait PaymentMethod {
    fn method(&self) -> Methods;
    fn to_serde_json_value(&self) -> Result<serde_json::Value>;
}

impl<'de, T> PaymentMethod for T
where
    T: MethodType + serde::de::DeserializeOwned + serde::Serialize,
{
    fn method(&self) -> Methods {
        T::type_method()
    }

    fn to_serde_json_value(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(&self)?)
    }
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

impl MethodType for PayPal {
    fn type_method() -> Methods {
        Methods::PayPal
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BtcLightning {
    keysend: String,
}

impl BtcLightning {
    pub fn new(keysend: &str) -> Result<Self> {
        Ok(Self {
            keysend: keysend.to_string(),
        })
    }
}

impl MethodType for BtcLightning {
    fn type_method() -> Methods {
        Methods::BtcLightning
    }
}

pub fn check(
    payment_methods: &std::collections::BTreeMap<Methods, serde_json::Value>,
) -> Result<()> {
    for (method, json_value) in payment_methods {
        let clean_json_value = match method {
            Methods::PayPal => {
                let method = serde_json::from_value::<PayPal>(json_value.clone())?;
                serde_json::to_value(&method)?
            }
            Methods::BtcLightning => {
                let method = serde_json::from_value::<BtcLightning>(json_value.clone())?;
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
