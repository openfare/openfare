use anyhow::{format_err, Result};

#[derive(
    Debug,
    Default,
    Clone,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Currency {
    #[default]
    USD,

    BTC,
}

impl std::convert::TryFrom<&str> for Currency {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_string().to_uppercase();
        Ok(match value.as_str() {
            "USD" => Self::USD,
            "BTC" => Self::BTC,
            _ => {
                return Err(format_err!("Unknown currency: {}", value));
            }
        })
    }
}

impl std::string::ToString for Currency {
    fn to_string(&self) -> String {
        match self {
            Self::USD => "USD",
            Self::BTC => "BTC",
        }
        .to_string()
    }
}

pub type Quantity = u64;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, serde::Serialize)]
pub struct Price {
    pub quantity: Quantity,
    pub currency: Currency,
}

struct Visitor {
    marker: std::marker::PhantomData<fn() -> Price>,
}

impl Visitor {
    fn new() -> Self {
        Visitor {
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Price;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string such as '50 USD'")
    }

    fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let re = regex::Regex::new(r"([0-9]+) ([A-Z]+)").map_err(|_| {
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
        let currency = parse_currency(&captures.get(2)).map_err(|_| {
            serde::de::Error::custom(serde::de::Unexpected::Other(
                format!("Failed to parse currency: {}", v).as_str(),
            ))
        })?;

        Ok(Self::Value { quantity, currency })
    }
}

impl<'de> serde::Deserialize<'de> for Price {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
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

fn parse_currency(regex_capture: &Option<regex::Match>) -> Result<Currency> {
    let error_message = "Failed to parse currency";
    let currency = regex_capture.ok_or(format_err!(error_message))?.as_str();

    let currency = match currency {
        "USD" => Currency::USD,
        "BTC" => Currency::BTC,
        _ => {
            return Err(format_err!(error_message));
        }
    };
    Ok(currency)
}

#[test]
fn test_price_correctly_parsed() -> anyhow::Result<()> {
    #[derive(Eq, PartialEq, serde::Deserialize)]
    struct Result {
        price: Price,
    }
    let result: Result = serde_json::from_str("{\"price\": \"50 USD\"}")?;
    let expected = Result {
        price: Price {
            quantity: 50,
            currency: Currency::USD,
        },
    };
    assert!(result == expected);
    Ok(())
}
