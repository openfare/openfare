#[derive(
    Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Config {
    #[serde(rename = "employees-count")]
    pub employees_count: Option<usize>,
}
