use super::common;
use anyhow::Result;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ForProfit {
    #[serde(rename = "for-profit")]
    pub state: bool,
}

impl ForProfit {
    pub fn new() -> Self {
        Self { state: true }
    }
}

impl common::Condition for ForProfit {
    fn evaluate(&self, parameters: &crate::lock::plan::conditions::Parameters) -> Result<bool> {
        let result = parameters
            .for_profit
            .ok_or(anyhow::format_err!("Unset parameter value: for-profit."))?;
        Ok(result)
    }

    fn metadata(&self) -> Box<dyn common::ConditionMetadata> {
        Box::new(ForProfitMetadata) as Box<dyn common::ConditionMetadata>
    }
}

#[derive(Debug, Clone)]
struct ForProfitMetadata;

impl common::ConditionMetadata for ForProfitMetadata {
    fn name(&self) -> String {
        "for-profit".to_string()
    }

    fn interactive_set_parameter(
        &self,
        parameters: &mut crate::lock::plan::conditions::Parameters,
    ) -> Result<()> {
        if dialoguer::Confirm::new()
            .with_prompt("Is the software used by a for-profit organization/individual?")
            .interact()?
        {
            parameters.for_profit = Some(true);
        } else {
            parameters.for_profit = Some(false);
        }
        Ok(())
    }

    fn is_parameter_set(&self, parameters: &crate::lock::plan::conditions::Parameters) -> bool {
        parameters.for_profit.is_some()
    }
}

#[test]
fn test_evaluate_cases() -> Result<()> {
    use common::Condition;

    let mut parameters = crate::lock::plan::conditions::Parameters::default();
    parameters.for_profit = Some(true);

    let for_profit = ForProfit::new();
    assert!(for_profit.evaluate(&parameters)?);
    Ok(())
}
