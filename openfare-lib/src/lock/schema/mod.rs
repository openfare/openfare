use anyhow::Result;

pub static VERSION: &str = "1";

lazy_static! {
    static ref SCHEMA: jsonschema::JSONSchema = {
        let schema = std::include_str!("schema.json");
        let schema = serde_json::from_str(schema).expect("serde parsed lock schema");
        jsonschema::JSONSchema::compile(&schema).expect("compiled lock schema")
    };
}

pub fn validate(value: &serde_json::Value) -> Result<()> {
    let result = SCHEMA.validate(&value);
    if let Err(errors) = result {
        let error_string = validation_errors_to_string(errors);
        return Err(anyhow::format_err!(
            "Invalid lock file\n".to_owned() + &error_string
        ));
    }
    Ok(())
}

fn validation_errors_to_string(errors: jsonschema::ErrorIterator) -> String {
    let mut error_string = String::new();
    error_string += &("----------------------\n");
    for error in errors {
        error_string += format!("Validation error: {}\n", error).as_str();
        error_string += format!("Instance path: {}\n", error.instance_path).as_str();
    }
    error_string
}
