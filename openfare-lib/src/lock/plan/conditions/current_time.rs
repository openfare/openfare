use super::common;
use anyhow::{format_err, Result};
use lazy_regex::regex;
use std::convert::TryFrom;

use chrono::{TimeZone, Utc};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CurrentTime {
    operator: common::Operator,
    time: chrono::DateTime<Utc>,
}

impl std::convert::TryFrom<&str> for CurrentTime {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (operator, time) = parse_value(&value)?;
        Ok(Self { operator, time })
    }
}

impl CurrentTime {
    pub fn evaluate(&self) -> Result<bool> {
        let current_time = chrono::offset::Utc::now();
        let result = common::evaluate_operator::<chrono::DateTime<Utc>>(
            &current_time,
            &self.operator,
            &self.time,
        );
        Ok(result)
    }
}

impl serde::Serialize for CurrentTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            format!(
                "{} {}",
                self.operator.to_string(),
                self.time.format("%Y-%m-%d"),
            )
            .as_str(),
        )
    }
}

struct Visitor {
    marker: std::marker::PhantomData<fn() -> CurrentTime>,
}

impl Visitor {
    fn new() -> Self {
        Visitor {
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = CurrentTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string such as '< 2022-01-31'")
    }

    fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let (operator, time) = parse_value(&value).expect("parse current-time condition value");
        Ok(Self::Value { operator, time })
    }
}

impl<'de> serde::Deserialize<'de> for CurrentTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::new())
    }
}

fn parse_value(value: &str) -> Result<(common::Operator, chrono::DateTime<Utc>)> {
    let re = regex!(r"(?P<operator>(>=)|(<=)|(<)|(>)|(=))\s*(?P<time>.*)");
    let captures = re
        .captures(value)
        .ok_or(format_err!("Regex failed to capture field."))?;

    let operator_match = captures
        .name("operator")
        .expect("extract operator from regex capture")
        .as_str();
    let operator = common::Operator::try_from(operator_match)?;

    let time_match = captures
        .name("time")
        .expect("extract time from regex capture")
        .as_str();
    let date = chrono::NaiveDate::parse_from_str(&time_match, "%Y-%m-%d")?;
    let time = naive_date_to_utc(&date)?;
    Ok((operator, time))
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
    let condition = CurrentTime::try_from("< 3022-01-31")?;
    assert!(condition.evaluate()?);
    Ok(())
}

#[test]
fn test_parse_str() -> Result<()> {
    let result = CurrentTime::try_from("< 2022-01-31")?;

    let expected_date = chrono::NaiveDate::parse_from_str("2022-01-31", "%Y-%m-%d")?;
    let expected = CurrentTime {
        operator: common::Operator::LessThan,
        time: naive_date_to_utc(&expected_date)?,
    };

    assert_eq!(result, expected);
    Ok(())
}
