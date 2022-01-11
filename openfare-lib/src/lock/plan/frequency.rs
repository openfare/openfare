use anyhow::{format_err, Result};

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Unit {
    Hours,
    Days,
    Months,
    Years,
    Once,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
pub struct Frequency {
    quantity: u64,
    unit: Unit,
}

struct Visitor {
    marker: std::marker::PhantomData<fn() -> Frequency>,
}

impl Visitor {
    fn new() -> Self {
        Visitor {
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Frequency;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string such as '30 days'")
    }

    fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let re = regex::Regex::new(r"([0-9]+) ([a-z]+)").map_err(|_| {
            serde::de::Error::custom(serde::de::Unexpected::Other("Code error: invalid regex."))
        })?;
        let captures =
            re.captures(v)
                .ok_or(serde::de::Error::custom(serde::de::Unexpected::Other(
                    format!("No regex captures found: {}", v).as_str(),
                )))?;

        let quantity = parse_quantity(&captures.get(1)).map_err(|_| {
            serde::de::Error::custom(serde::de::Unexpected::Other(
                format!("Failed to parse quantity: {}", v).as_str(),
            ))
        })?;
        let unit = parse_unit(&captures.get(2)).map_err(|_| {
            serde::de::Error::custom(serde::de::Unexpected::Other(
                format!("Failed to parse unit: {}", v).as_str(),
            ))
        })?;

        Ok(Self::Value { quantity, unit })
    }
}

// This is the trait that informs Serde how to deserialize MyMap.
impl<'de> serde::Deserialize<'de> for Frequency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of MyMap.
        deserializer.deserialize_str(Visitor::new())
    }
}

fn parse_quantity(regex_capture: &Option<regex::Match>) -> Result<u64> {
    let quantity = regex_capture
        .ok_or(format_err!("Failed to parse quantity"))?
        .as_str();
    let quantity = quantity.parse::<u64>()?;
    Ok(quantity)
}

fn parse_unit(regex_capture: &Option<regex::Match>) -> Result<Unit> {
    let error_message = "Failed to parse currency";
    let unit = regex_capture.ok_or(format_err!(error_message))?.as_str();

    let unit = match unit {
        "hours" => Unit::Hours,
        "days" => Unit::Days,
        "months" => Unit::Months,
        "years" => Unit::Years,
        "once" => Unit::Once,
        _ => {
            return Err(format_err!(error_message));
        }
    };
    Ok(unit)
}

#[test]
fn test_frequency_correctly_parsed() -> anyhow::Result<()> {
    #[derive(Eq, PartialEq, serde::Deserialize)]
    struct Result {
        frequency: Frequency,
    }
    let result: Result = serde_json::from_str("{\"frequency\": \"50 days\"}")?;
    let expected = Result {
        frequency: Frequency {
            quantity: 50,
            unit: Unit::Days,
        },
    };
    assert!(result == expected);
    Ok(())
}
