use anyhow::Result;

pub mod payee;
pub mod plan;
pub mod shares;

pub static FILE_NAME: &str = "OpenFare.lock";
pub static SCHEME_VERSION: &str = "1";

lazy_static! {
    static ref SCHEMA: jsonschema::JSONSchema = {
        let schema = std::include_str!("schema.json");
        let schema = serde_json::from_str(schema).expect("serde parsed lock schema");
        jsonschema::JSONSchema::compile(&schema).expect("compiled lock schema")
    };
}

/// A software package's OpenFare lock file (OpenFare.lock).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lock {
    #[serde(rename = "scheme-version")]
    pub scheme_version: String,
    pub plans: plan::Plans,
    pub payees: payee::Payees,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<shares::Shares>,
}

impl Lock {
    /// Validate lock against schema.
    pub fn validate(&self) -> Result<()> {
        let value = serde_json::to_value(&self)?;
        let result = SCHEMA.validate(&value);
        if let Err(errors) = result {
            let error_string = lockfile_errors_to_string(errors);
            return Err(anyhow::format_err!(
                "Invalid lockfile\n".to_owned() + &error_string
            ));
        }
        Ok(())
    }
}

fn lockfile_errors_to_string(errors: jsonschema::ErrorIterator) -> String {
    let mut error_string = String::new();
    error_string += &("----------------------\n");
    for error in errors {
        error_string += format!("Validation error: {}\n", error).as_str();
        error_string += format!("Instance path: {}\n", error.instance_path).as_str();
    }
    error_string
}

impl std::default::Default for Lock {
    fn default() -> Self {
        Self {
            scheme_version: SCHEME_VERSION.to_string(),
            plans: plan::Plans::new(),
            payees: payee::Payees::new(),
            shares: None,
        }
    }
}

#[test]
fn test_serialize() -> anyhow::Result<()> {
    let mut lock = Lock::default();

    let mut plan = plan::Plan {
        r#type: plan::PlanType::Voluntary,
        conditions: plan::conditions::Conditions::default(),
        price: Some(crate::price::Price::default()),
    };

    plan.conditions.employees_count = Some(plan::conditions::EmployeesCount::try_from(
        "1 <= count < 50",
    )?);
    lock.plans.insert("0".to_string(), plan);

    serde_json::to_string_pretty(&lock)?;
    Ok(())
}
