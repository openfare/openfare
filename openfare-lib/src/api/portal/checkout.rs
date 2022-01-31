use super::common;

lazy_static! {
    pub static ref ROUTE: String = format!("{}/checkout", super::ROUTE.as_str());
}

pub type ExtensionName = String;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Order {
    pub items: std::collections::BTreeMap<ExtensionName, Vec<PackagePlans>>,
    pub api_key: common::ApiKey,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PackagePlans {
    pub package: crate::package::Package,
    pub plans: Vec<Plan>,
    pub payees: crate::lock::payee::Payees,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Plan {
    pub plan_id: String,
    pub plan: crate::lock::plan::Plan,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Response {
    pub checkout_url: url::Url,
}
