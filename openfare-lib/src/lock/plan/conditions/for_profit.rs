use anyhow::Result;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForProfit {}

impl ForProfit {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&self, parameters: &crate::lock::plan::conditions::Parameters) -> Result<bool> {
        let result = parameters
            .for_profit
            .ok_or(anyhow::format_err!("Unset parameter value: for-profit."))?;
        Ok(result)
    }
}

impl serde::Serialize for ForProfit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("true")
    }
}

struct Visitor {
    marker: std::marker::PhantomData<fn() -> ForProfit>,
}

impl Visitor {
    fn new() -> Self {
        Visitor {
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = ForProfit;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("the string 'true'")
    }

    fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value != "true" {
            Err(E::custom(format!(
                "Unexpected 'for-profit' value: {}",
                value
            )))
        } else {
            Ok(Self::Value {})
        }
    }
}

impl<'de> serde::Deserialize<'de> for ForProfit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::new())
    }
}

#[test]
fn test_evaluate_cases() -> Result<()> {
    let mut parameters = crate::lock::plan::conditions::Parameters::default();
    parameters.for_profit = Some(true);

    let for_profit = ForProfit::new();
    assert!(for_profit.evaluate(&parameters)?);
    Ok(())
}
