#[derive(
    Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Config {
    #[serde(rename = "developers-count")]
    pub developers_count: Option<u64>,
}
