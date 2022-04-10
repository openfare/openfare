use super::common;
use anyhow::Result;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Parameters {
    #[serde(rename = "employees-count")]
    pub employees_count: Option<super::employees_count::Range>,

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

/// Check correct parameters set for the given package locks conditions.
/// Attempts to set parameters if they are not set.
///
/// Returns true if parameters were set, false if they required modification.
pub fn check_set(
    conditions_metadata: &Vec<Box<dyn common::ConditionMetadata>>,
    parameters: &mut Parameters,
) -> Result<bool> {
    let mut parameter_set_correct = true;
    for metadata in conditions_metadata {
        log::info!("Checking condition {}", metadata.name());
        if !metadata.is_parameter_set(&parameters) {
            println!(
                "Please set profile condition parameter: {}",
                metadata.name()
            );
            metadata.interactive_set_parameter(parameters)?;
            parameter_set_correct = false;
        }
    }
    Ok(parameter_set_correct)
}
