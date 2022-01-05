use super::common;
use anyhow::{format_err, Result};
use lazy_regex::regex;
use std::convert::TryFrom;

pub fn evaluate(value: &str) -> Result<bool> {
    let (operator, time) = parse_value(&value)?;
    let current_time = chrono::offset::Utc::now();
    let result =
        common::evaluate_operator::<chrono::DateTime<chrono::Utc>>(&current_time, &operator, &time);
    Ok(result)
}

fn parse_value(value: &str) -> Result<(common::Operator, chrono::DateTime<chrono::Utc>)> {
    let re = regex!(r"(?P<operator>(>=)|(<=)|(<)|(>)|(=)) (?P<quantity>.*)");
    let captures = re
        .captures(value)
        .ok_or(format_err!("Regex failed to capture field."))?;

    let operator_match = captures
        .name("operator")
        .expect("extract operator from regex capture")
        .as_str();
    let operator = common::Operator::try_from(operator_match)?;

    let time_match = captures
        .name("quantity")
        .expect("extract time from regex capture")
        .as_str();
    let time = chrono::DateTime::parse_from_rfc3339(&time_match)?.with_timezone(&chrono::Utc);

    Ok((operator, time))
}

#[test]
fn test_evaluate_cases() -> Result<()> {
    let time = chrono::offset::Utc::now() + chrono::Duration::days(10);
    assert!(evaluate(&format!("< {}", time.to_rfc3339()))?);
    Ok(())
}
