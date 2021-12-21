use anyhow::Result;

pub mod conditions;
mod frequency;
pub mod price;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Recipient {
    name: Option<String>,
    address: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payment {
    recipient: Recipient,
    price: price::Price,

    #[serde(flatten)]
    method: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentPlan {
    pub conditions: std::collections::BTreeMap<conditions::Condition, String>,
    pub payments: Vec<Payment>,
    pub frequency: frequency::Frequency,
}

impl PaymentPlan {
    pub fn is_applicable(&self, config: &crate::config::Config) -> Result<bool> {
        let mut all_conditions_pass = true;
        for (condition, value) in &self.conditions {
            all_conditions_pass &= conditions::evaluate(&condition, &value, &config)?;
        }
        Ok(all_conditions_pass)
    }

    pub fn total_price(&self) -> Result<price::Quantity> {
        let mut currency_totals = std::collections::BTreeMap::<price::Currency, u64>::new();
        for payment in &self.payments {
            let quantity = currency_totals
                .entry(payment.price.currency.clone())
                .or_insert_with(|| payment.price.quantity);
            *quantity += payment.price.quantity;
        }
        // TODO: Include non-USD currencies in price.
        Ok(*currency_totals.get(&price::Currency::USD).unwrap_or(&0))
    }
}
