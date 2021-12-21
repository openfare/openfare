use anyhow::Result;

mod common;
mod current_time_threshold;
mod developers_count;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum Condition {
    #[serde(rename = "developers-count")]
    DevelopersCount,

    #[serde(rename = "current-time-threshold")]
    CurrentTimeThreshold,
}

pub fn evaluate(
    condition: &Condition,
    value: &str,
    config: &crate::config::Config,
) -> Result<bool> {
    Ok(match condition {
        Condition::DevelopersCount => developers_count::evaluate(&value, &config)?,
        Condition::CurrentTimeThreshold => current_time_threshold::evaluate(&value, &config)?,
    })
}
