use anyhow::Result;

pub fn evaluate(value: &str, _config: &crate::config::Config) -> Result<bool> {
    let value = chrono::DateTime::parse_from_rfc3339(&value)?;
    let current_time = chrono::offset::Utc::now();
    Ok(current_time < value)
}

#[test]
fn test_evaluate_cases() -> Result<()> {
    let config = crate::config::Config::default();
    assert_eq!(evaluate("1996-12-19T16:39:57-00:00", &config)?, false);
    Ok(())
}
