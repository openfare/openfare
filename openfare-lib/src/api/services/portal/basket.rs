lazy_static! {
    pub static ref ROUTE: String = format!("{}/basket", super::ROUTE.as_str());
}
pub use super::super::basket::Item;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Order {
    pub items: Vec<Item>,
    pub api_key: super::ApiKey,
}

impl Order {
    /// Order is empty if no plan in any item.
    pub fn is_empty(&self) -> bool {
        self.items.iter().all(|item| item.plans.is_empty())
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Response {
    pub checkout_url: url::Url,
}
