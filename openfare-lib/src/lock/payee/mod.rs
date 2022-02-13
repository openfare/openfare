use anyhow::{format_err, Result};
pub mod payment_methods;

pub type Name = String;
pub type Payees = std::collections::BTreeMap<Name, Payee>;
pub type PaymentMethodName = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payee {
    pub label: Name,
    pub url: String,
    #[serde(rename = "unique-id")]
    pub unique_id: uuid::Uuid,
    #[serde(rename = "payment-methods")]
    payment_methods: std::collections::BTreeMap<PaymentMethodName, serde_json::Value>,
}

impl Payee {
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

impl std::default::Default for Payee {
    fn default() -> Self {
        Self {
            label: "".to_string(),
            url: "".to_string(),
            unique_id: uuid::Uuid::new_v4(),
            payment_methods: std::collections::BTreeMap::<_, _>::new(),
        }
    }
}

pub fn unique_name(payee_name: &Name, payee: &Payee) -> Name {
    let unique_id = payee.unique_id.to_string()[..13].to_string();
    format!("{payee_name}___{unique_id}")
}

pub fn get_lock_payee(
    payee: &Payee,
    all_lock_payees: &std::collections::BTreeMap<Name, Payee>,
) -> Option<(Name, Payee)> {
    for (name, existing_payee) in all_lock_payees {
        if payee.unique_id == existing_payee.unique_id {
            return Some((name.clone(), existing_payee.clone()));
        }
    }
    None
}
