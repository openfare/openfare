#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Parameters {
    #[serde(rename = "employees-count")]
    pub employees_count: Option<usize>,

    #[serde(rename = "for-profit")]
    pub for_profit: Option<bool>,

    #[serde(rename = "include-voluntary-donations")]
    pub include_voluntary_plans: bool,
}

impl std::default::Default for Parameters {
    fn default() -> Self {
        Self {
            employees_count: None,
            for_profit: None,
            include_voluntary_plans: true,
        }
    }
}

impl std::fmt::Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}
