use anyhow::{format_err, Result};
use std::str::FromStr;

#[derive(
    Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub enum Currency {
    USD,
    BTC,
}

impl Currency {
    pub fn decimal_points(&self) -> u32 {
        match self {
            Self::USD => 2,
            Self::BTC => 8,
        }
    }
}

impl std::default::Default for Currency {
    fn default() -> Self {
        Self::USD
    }
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

impl std::fmt::Display for Currency {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let currency = match self {
            Self::USD => "USD",
            Self::BTC => "BTC",
        };
        write!(formatter, "{}", currency)
    }
}

pub type Quantity = rust_decimal::Decimal;

#[derive(Debug, Default, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Price {
    pub quantity: Quantity,
    pub currency: Currency,
}

impl std::iter::Sum for Price {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut currency = None;
        let mut quantity = Quantity::from(0 as i64);
        for price in iter {
            quantity += price.quantity;
            currency = Some(price.currency);
        }
        Self {
            quantity,
            currency: currency.unwrap_or_default(),
        }
    }
}

impl std::convert::TryFrom<&str> for Price {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        #[derive(Eq, PartialEq, serde::Deserialize)]
        struct Result {
            price: Price,
        }
        let result: Result =
            serde_json::from_str(format!("{{\"price\": \"{}\"}}", value).as_str())?;
        let mut price = result.price;
        price.quantity = price.quantity.round_dp_with_strategy(
            price.currency.decimal_points(),
            rust_decimal::prelude::RoundingStrategy::AwayFromZero,
        );
        Ok(price)
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} {}", self.quantity, self.currency)
    }
}

impl serde::Serialize for Price {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            format!(
                "{:.1$} {currency}",
                self.quantity,
                self.currency.decimal_points() as usize,
                currency = self.currency.to_string(),
            )
            .as_str(),
        )
    }
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
        let re = regex::Regex::new(r"([0-9]+[\.]?[0-9]*)\s*([a-zA-Z]+)").map_err(|_| {
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

fn parse_quantity(regex_capture: &Option<regex::Match>) -> Result<rust_decimal::Decimal> {
    let quantity = regex_capture
        .ok_or(format_err!("Failed to parse quantity"))?
        .as_str();
    let quantity = rust_decimal::Decimal::from_str(quantity)?;
    Ok(quantity)
}

fn parse_currency(regex_capture: &Option<regex::Match>) -> Result<Currency> {
    let error_message = "Failed to parse currency";
    let currency = regex_capture.ok_or(format_err!(error_message))?.as_str();

    let currency = match currency.to_uppercase().as_str() {
        "USD" => Currency::USD,
        "BTC" => Currency::BTC,
        _ => {
            return Err(format_err!(error_message));
        }
    };
    Ok(currency)
}

#[test]
fn test_serialize_usd() -> anyhow::Result<()> {
    #[derive(serde::Serialize)]
    struct Tmp {
        price: Price,
    }
    let t = Tmp {
        price: Price::try_from("50   usd")?,
    };
    let result = serde_json::to_string(&t)?;
    let expected = "{\"price\":\"50.00 USD\"}".to_string();
    assert!(result == expected);
    Ok(())
}

#[test]
fn test_serialize_btc() -> anyhow::Result<()> {
    #[derive(serde::Serialize)]
    struct Tmp {
        price: Price,
    }
    let t = Tmp {
        price: Price::try_from("50   btc")?,
    };
    let result = serde_json::to_string(&t)?;
    let expected = "{\"price\":\"50.00000000 BTC\"}".to_string();
    println!("{}", result);
    assert!(result == expected);
    Ok(())
}

#[test]
fn test_str_price_correctly_parsed() -> anyhow::Result<()> {
    let result = Price::try_from("50   usd")?;
    let expected = Price {
        quantity: rust_decimal::Decimal::from(50),
        currency: Currency::USD,
    };
    assert!(result == expected);
    Ok(())
}

#[test]
fn test_decimal_price_correctly_parsed() -> anyhow::Result<()> {
    let result = Price::try_from("50.02   usd")?;
    let expected = Price {
        quantity: rust_decimal::Decimal::from_str("50.02")?,
        currency: Currency::USD,
    };
    assert!(result == expected);
    Ok(())
}
