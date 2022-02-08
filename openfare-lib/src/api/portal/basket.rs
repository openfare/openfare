use super::common;

lazy_static! {
    pub static ref ROUTE: String = format!("{}/basket", super::ROUTE.as_str());
}

pub type ExtensionName = String;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Order {
    pub items: std::collections::BTreeMap<ExtensionName, Vec<PackagePlans>>,
    pub api_key: common::ApiKey,
}

impl Order {
    /// Order is empty if it does not include and payment plans.
    pub fn is_empty(&self) -> bool {
        self.items.iter().all(|(_, all_package_plans)| {
            all_package_plans
                .iter()
                .all(|package_plans| package_plans.plans.is_empty())
        })
    }
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
