pub mod payee;
pub mod plan;
pub mod shares;

pub static FILE_NAME: &str = "OpenFare.lock";
pub static SCHEME_VERSION: &str = "1";

lazy_static! {
    pub static ref SCHEMA: jsonschema::JSONSchema = {
        let schema = std::include_str!("schema.json");
        let schema = serde_json::from_str(schema).expect("serde parsed lock schema");
        jsonschema::JSONSchema::compile(&schema).expect("compiled lock schema")
    };
}

/// A software package's OpenFare lock file (OpenFare.lock).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Lock {
    #[serde(rename = "scheme-version")]
    pub scheme_version: String,
    pub plans: plan::Plans,
    pub payees: payee::Payees,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<shares::Shares>,
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
