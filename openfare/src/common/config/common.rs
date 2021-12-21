use anyhow::{format_err, Result};

pub fn bool_from_string(value: &str) -> Result<bool> {
    Ok(match value {
        "true" => true,
        "false" => false,
        _ => {
            return Err(format_err!(
                "Expected value: `true` or `false`. Found: {}",
                value
            ));
        }
    })
}
