pub type Label = String;

pub type PaymentMethod = serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payee {
    #[serde(rename = "payment-methods")]
    pub payment_methods: Vec<PaymentMethod>,
}
