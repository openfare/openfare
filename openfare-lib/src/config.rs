// TODO: Rename to condition parameters.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(rename = "employees-count")]
    pub employees_count: Option<usize>,
    pub commercial: bool,
    #[serde(rename = "include-voluntary-donations")]
    pub include_voluntary_plans: bool,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            employees_count: None,
            commercial: true,
            include_voluntary_plans: true,
        }
    }
}
