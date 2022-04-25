use anyhow::Result;
pub mod payment_methods;

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
#[serde(deny_unknown_fields)]
pub struct Profile {
    #[serde(rename = "unique-id")]
    pub unique_id: uuid::Uuid,
    #[serde(rename = "payment-methods")]
    pub payment_methods: std::collections::BTreeMap<payment_methods::Methods, serde_json::Value>,
}

impl Profile {
    pub fn check(&mut self) -> Result<()> {
        payment_methods::check(&self.payment_methods)?;
        Ok(())
    }

    pub fn payment_methods(&self) -> Result<Vec<Box<dyn payment_methods::PaymentMethod>>> {
        let mut methods = Vec::<Box<dyn payment_methods::PaymentMethod>>::new();
        for (method, json_value) in &self.payment_methods {
            let method = match method {
                payment_methods::Methods::PayPal => {
                    let method =
                        serde_json::from_value::<payment_methods::PayPal>(json_value.clone())?;
                    Box::new(method) as Box<dyn payment_methods::PaymentMethod>
                }
                payment_methods::Methods::BtcLightning => {
                    let method = serde_json::from_value::<payment_methods::BtcLightning>(
                        json_value.clone(),
                    )?;
                    Box::new(method) as Box<dyn payment_methods::PaymentMethod>
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
        self.payment_methods.insert(
            payment_method.method(),
            payment_method.to_serde_json_value()?,
        );
        Ok(())
    }

    pub fn remove_payment_method(&mut self, method: &payment_methods::Methods) -> Result<()> {
        self.payment_methods.remove(method);
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
