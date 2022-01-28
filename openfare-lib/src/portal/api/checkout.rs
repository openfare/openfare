use super::common;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Order {
    pub items: Vec<OrderItem>,
    pub api_key: common::ApiKey,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct OrderItem {
    pub package: common::Package,
    pub plan: common::Plan,
}
