use super::common;

lazy_static! {
    pub static ref ROUTE: String = format!("{}/basket", super::ROUTE.as_str());
}

pub type ExtensionName = String;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Order {
    pub items: Vec<Item>,
    pub api_key: common::ApiKey,
}

impl Order {
    /// Order is empty if no plan in any item.
    pub fn is_empty(&self) -> bool {
        self.items.iter().all(|item| item.plans.is_empty())
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Item {
    pub package: crate::package::Package,
    pub extension_name: ExtensionName,
    pub plans: Vec<Plan>,
    pub total_price: crate::lock::plan::price::Price,
    pub payees: crate::lock::payee::Payees,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Plan {
    pub plan_id: String,
    pub plan: crate::lock::plan::Plan,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Response {
    pub checkout_url: url::Url,
}
