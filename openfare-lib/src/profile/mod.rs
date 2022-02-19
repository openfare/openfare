use anyhow::{format_err, Result};
pub mod payment_methods;

pub type PaymentMethodName = String;

pub const FILE_NAME: &'static str = "openfare-profile.json";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteProfile {
    #[serde(rename = "openfare-profile")]
    pub profile: Profile,
}

impl std::convert::From<Profile> for RemoteProfile {
    fn from(profile: Profile) -> Self {
        Self { profile: profile }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    #[serde(rename = "unique-id")]
    pub unique_id: uuid::Uuid,
    #[serde(rename = "payment-methods")]
    payment_methods: std::collections::BTreeMap<PaymentMethodName, serde_json::Value>,
}

impl Profile {
    pub fn payment_methods(&self) -> Result<Vec<Box<dyn payment_methods::PaymentMethod>>> {
        let mut methods = Vec::<Box<dyn payment_methods::PaymentMethod>>::new();
        for (name, json_value) in &self.payment_methods {
            let method = match name.as_str() {
                "paypal" => Box::new(serde_json::value::from_value::<payment_methods::PayPal>(
                    json_value.clone(),
                )?) as Box<dyn payment_methods::PaymentMethod>,
                "btc_lightning_keysend" => Box::new(serde_json::value::from_value::<
                    payment_methods::BtcLightningKeysend,
                >(json_value.clone())?)
                    as Box<dyn payment_methods::PaymentMethod>,
                _ => {
                    return Err(format_err!("Unknown payment method: {}", name));
                }
            };
            methods.push(method);
        }
        Ok(methods)
    }

    pub fn set_payment_method(
        &mut self,
        payment_method: &Box<dyn payment_methods::PaymentMethod>,
    ) -> Result<()> {
        let name = payment_method.name();
        self.payment_methods
            .insert(name, payment_method.to_serde_json_value()?);
        Ok(())
    }

    pub fn remove_payment_method(&mut self, name: &String) -> Result<()> {
        self.payment_methods.remove(name);
        Ok(())
    }
}

impl std::default::Default for Profile {
    fn default() -> Self {
        Self {
            unique_id: uuid::Uuid::new_v4(),
            payment_methods: std::collections::BTreeMap::<_, _>::new(),
        }
    }
}

impl std::convert::From<RemoteProfile> for Profile {
    fn from(remote_profile: RemoteProfile) -> Self {
        remote_profile.profile.clone()
    }
}
