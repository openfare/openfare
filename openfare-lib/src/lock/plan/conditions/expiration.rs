use super::common;
use anyhow::Result;

use chrono::{TimeZone, Utc};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Expiration {
    time: chrono::DateTime<Utc>,
}

impl std::convert::TryFrom<&str> for Expiration {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let time = parse_value(&value)?;
        Ok(Self { time })
    }
}

impl common::Condition for Expiration {
    fn evaluate(&self, _parameters: &crate::lock::plan::conditions::Parameters) -> Result<bool> {
        let current_time = chrono::offset::Utc::now();
        let expiration = &self.time;
        let result = common::evaluate_operator::<chrono::DateTime<Utc>>(
            &current_time,
            &common::Operator::LessThan,
            &expiration,
        );
        Ok(result)
    }

    fn metadata(&self) -> Box<dyn common::ConditionMetadata> {
        Box::new(ExpirationMetadata) as Box<dyn common::ConditionMetadata>
    }
}

impl serde::Serialize for Expiration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{}", self.time.format("%Y-%m-%d"),).as_str())
    }
}

struct Visitor {
    marker: std::marker::PhantomData<fn() -> Expiration>,
}

impl Visitor {
    fn new() -> Self {
        Visitor {
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Expiration;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string such as '2022-01-31'")
    }

    fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let time = parse_value(&value).expect("parse expiration condition value");
        Ok(Self::Value { time })
    }
}

impl<'de> serde::Deserialize<'de> for Expiration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::new())
    }
}

#[derive(Debug, Clone)]
struct ExpirationMetadata;

impl common::ConditionMetadata for ExpirationMetadata {
    fn name(&self) -> String {
        "expiration".to_string()
    }

    fn interactive_set_parameter(
        &self,
        _parameters: &mut crate::lock::plan::conditions::Parameters,
    ) -> Result<()> {
        Ok(())
    }

    fn is_parameter_set(&self, _parameters: &crate::lock::plan::conditions::Parameters) -> bool {
        true
    }
}

fn parse_value(value: &str) -> Result<chrono::DateTime<Utc>> {
    let date = chrono::NaiveDate::parse_from_str(&value, "%Y-%m-%d")?;
    let time = naive_date_to_utc(&date)?;
    Ok(time)
}

fn naive_date_to_utc(date: &chrono::NaiveDate) -> Result<chrono::DateTime<Utc>> {
    // The known 1 hour time offset in seconds
    let tz_offset = chrono::FixedOffset::east(0);
    // The known time
    let time = chrono::NaiveTime::from_hms(0, 0, 0);
    // Naive date time, with no time zone information
    let datetime = chrono::NaiveDateTime::new(date.clone(), time);

    let dt_with_tz: chrono::DateTime<chrono::FixedOffset> =
        tz_offset.from_local_datetime(&datetime).unwrap();
    let dt_with_tz_utc: chrono::DateTime<Utc> = Utc.from_utc_datetime(&dt_with_tz.naive_utc());
    Ok(dt_with_tz_utc)
}

#[test]
fn test_evaluate_cases() -> Result<()> {
    use common::Condition;
    let condition = Expiration::try_from("3022-01-31")?;
    let parameters = crate::lock::plan::conditions::Parameters::default();
    assert!(condition.evaluate(&parameters)?);
    Ok(())
}

#[test]
fn test_parse_str() -> Result<()> {
    let result = Expiration::try_from("2022-01-31")?;

    let expected_date = chrono::NaiveDate::parse_from_str("2022-01-31", "%Y-%m-%d")?;
    let expected = Expiration {
        time: naive_date_to_utc(&expected_date)?,
    };

    assert_eq!(result, expected);
    Ok(())
}
