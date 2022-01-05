use super::common;
use anyhow::{format_err, Result};
use lazy_regex::regex;
use std::convert::TryFrom;

pub fn evaluate(value: &str, config: &crate::config::Config) -> Result<bool> {
    let (operator, quantity) = parse_value(value)?;
    let developers_count = config.developers_count.ok_or(format_err!(
        "Attempting to evaluate condition but the `developers-count` metric is unset."
    ))?;
    let result = common::evaluate_operator::<u64>(developers_count, &operator, quantity);
    Ok(result)
}

fn parse_value(value: &str) -> Result<(common::Operator, u64)> {
    let re = regex!(r"(?P<operator>(>=)|(<=)|(<)|(>)|(=))\s*(?P<quantity>[0-9]+)");
    let captures = re
        .captures(value)
        .ok_or(format_err!("Regex failed to capture field."))?;

    let operator_match = captures
        .name("operator")
        .expect("extract operator from regex capture")
        .as_str();
    let operator = common::Operator::try_from(operator_match)?;

    let quantity_match = captures
        .name("quantity")
        .expect("extract operator from regex capture")
        .as_str();
    let quantity = quantity_match.parse::<u64>()?;

    Ok((operator, quantity))
}

#[test]
fn test_evaluate_cases() -> Result<()> {
    let mut config = crate::config::Config::default();
    config.developers_count = Some(99);

    assert!(evaluate("<= 100", &config)?);
    assert!(evaluate("= 99", &config)?);
    assert!(evaluate("> 98", &config)?);
    assert!(evaluate(">= 99", &config)?);
    Ok(())
}
