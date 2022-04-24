use crate::command::lock::common;
use crate::handles::lock;
use anyhow::Result;
use serde_json::Value;

pub fn validate_lock_file(maybe_lock_file_path: &common::LockFilePathArg) -> Result<()> {
    let lock_file_pathbuf = get_lock_file_pathbuf(maybe_lock_file_path)?;

    let lock_file_string = file_to_string(lock_file_pathbuf.to_str().unwrap());
    validate_lock_file_string(lock_file_string)
}

pub fn validate_lock_file_json(lock_file_json: Value) -> Result<()> {
    let lock: openfare_lib::lock::Lock = serde_json::from_value(lock_file_json)?;
    lock.validate()
}

pub fn validate_lock_file_string(lock_file_string: String) -> Result<()> {
    validate_lock_file_json(serde_json::from_str(&lock_file_string).unwrap())
}

fn get_lock_file_pathbuf(
    maybe_lock_file_path: &common::LockFilePathArg,
) -> Result<std::path::PathBuf> {
    let lockfile_pathbuf = match &maybe_lock_file_path.path {
        None => lock::find_lock_file()?.unwrap(),
        Some(a) => a.to_path_buf(),
    };
    Ok(lockfile_pathbuf)
}

fn file_to_string(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use openfare_lib::lock::payee::Payee;
    use openfare_lib::lock::plan::conditions::{Conditions, ForProfit};
    use openfare_lib::lock::plan::{Plan, PlanType};
    use openfare_lib::lock::{shares, Lock};
    use openfare_lib::price::{Currency, Price};
    use openfare_lib::profile::Profile;
    use serde_json::json;

    fn generate_minimal_valid_lockfile() -> Value {
        let mut lock = Lock::default();
        lock.plans.insert(
            "0".to_string(),
            Plan {
                r#type: PlanType::Voluntary,
                conditions: Conditions {
                    for_profit: None,
                    expiration: None,
                    employees_count: None,
                },
                price: None,
            },
        );

        lock.payees.insert(
            "steve".to_string(),
            Payee {
                url: None,
                profile: Profile {
                    unique_id: Default::default(),
                    payment_methods: Default::default(),
                },
            },
        );

        let mut shares: shares::Shares = Default::default();
        shares.insert("steve".to_string(), 1);
        lock.shares = Some(shares);

        serde_json::to_value(&lock).unwrap()
    }
    fn generate_test_lockfile() -> Value {
        let mut lock = Lock::default();
        lock.plans.insert(
            "0".to_string(),
            Plan {
                r#type: PlanType::Compulsory,
                conditions: Conditions {
                    for_profit: Some(ForProfit { state: true }),
                    expiration: None,
                    employees_count: None,
                },
                price: Some(Price {
                    quantity: rust_decimal::Decimal::from(5),
                    currency: Currency::USD,
                }),
            },
        );
        lock.plans.insert(
            "1".to_string(),
            Plan {
                r#type: PlanType::Voluntary,
                conditions: Conditions {
                    for_profit: Some(ForProfit { state: false }),
                    expiration: None,
                    employees_count: None,
                },
                price: None,
            },
        );

        lock.payees.insert(
            "steve".to_string(),
            Payee {
                url: Some("github.com/steve".to_string()),
                profile: Profile {
                    unique_id: Default::default(),
                    payment_methods: Default::default(),
                },
            },
        );
        lock.payees.insert(
            "notSteve".to_string(),
            Payee {
                url: None,
                profile: Profile {
                    unique_id: Default::default(),
                    payment_methods: Default::default(),
                },
            },
        );
        let mut shares: shares::Shares = Default::default();
        shares.insert("steve".to_string(), 1000);
        shares.insert("notSteve".to_string(), 200);
        lock.shares = Some(shares);

        serde_json::to_value(&lock).unwrap()
    }
    fn generate_test_lockfile_with_typo() -> Value {
        let mut lockfile = generate_test_lockfile();
        let steve = lockfile["payees"]["steve"].as_object_mut().unwrap();
        if let Some(url) = steve.remove("url") {
            steve.insert("ur1".to_string(), url);
        } else {
            panic!("Adjustment failed");
        }
        println!("{}", serde_json::to_string_pretty(&lockfile).unwrap());
        lockfile
    }
    fn generate_test_lockfile_with_non_number_plan_key() -> Value {
        let mut lockfile = generate_test_lockfile();
        let plans = lockfile["plans"].as_object_mut().unwrap();
        if let Some(plan0) = plans.remove("0") {
            plans.insert("this_is_a_non_number_plan_key".to_string(), plan0);
        } else {
            panic!("Adjustment failed");
        }
        lockfile
    }
    fn generate_test_lockfile_with_past_zero_plan_key() -> Value {
        let mut lockfile = generate_test_lockfile();
        let plans = lockfile["plans"].as_object_mut().unwrap();
        if let Some(plan0) = plans.remove("0") {
            plans.insert("9001".to_string(), plan0);
        } else {
            panic!("Adjustment failed");
        }
        lockfile
    }
    fn generate_test_lockfile_with_negative_shares() -> Value {
        let mut lockfile = generate_test_lockfile();
        lockfile["shares"]
            .as_object_mut()
            .unwrap()
            .insert("steve".to_string(), json!(-50));
        println!("{}", serde_json::to_string_pretty(&lockfile).unwrap());
        lockfile
    }
    fn generate_test_lockfile_with_invalid_plan_condition() -> Value {
        let mut lockfile = generate_test_lockfile();
        lockfile["plans"]["0"]["conditions"]
            .as_object_mut()
            .unwrap()
            .insert(
                "fake_condition".to_string(),
                Value::String(
                    "THIS SOFTWARE CANNOT BE USED FOR ANY PURPOSE ON WENSDAYS".to_string(),
                ),
            );
        lockfile
    }
    fn generate_test_lockfile_with_more_share_labels_than_payees() -> Value {
        let mut lockfile = generate_test_lockfile();
        lockfile["shares"]
            .as_object_mut()
            .unwrap()
            .insert("otherNotSteve".to_string(), json!(5));
        lockfile
    }
    fn generate_test_lockfile_with_incorrect_share_label() -> Value {
        let mut lockfile = generate_test_lockfile();
        let shares = lockfile["shares"].as_object_mut().unwrap();
        if let Some(steves_shares) = shares.remove("steve") {
            shares.insert("someGuy".to_string(), steves_shares);
        } else {
            panic!("Adjustment failed");
        }
        lockfile
    }

    fn print_if_err(result: Result<()>) -> Result<()> {
        match &result {
            Ok(..) => {}
            Err(e) => println!("{}", e),
        }
        result
    }

    fn validate_lock_file_json_and_print_errs(val: Value) -> Result<()> {
        print_if_err(validate_lock_file_json(val))
    }

    #[test]
    fn test_empty_lockfile_not_valid() {
        assert!(validate_lock_file_json_and_print_errs(json!("")).is_err());
    }
    #[test]
    fn test_almost_empty_lockfile_not_valid() {
        assert!(validate_lock_file_json_and_print_errs(json!("{}")).is_err());
    }
    #[test]
    fn test_minimal_lock_file_is_valid() {
        assert!(validate_lock_file_json_and_print_errs(generate_minimal_valid_lockfile()).is_ok());
    }
    #[test]
    fn test_more_complex_lock_file_is_valid() {
        assert!(validate_lock_file_json_and_print_errs(generate_test_lockfile()).is_ok());
    }
    #[test]
    fn test_minor_typo_not_valid() {
        assert!(
            validate_lock_file_json_and_print_errs(generate_test_lockfile_with_typo()).is_err()
        );
    }
    #[test]
    fn test_plan_numbers_can_start_past_zero() {
        assert!(validate_lock_file_json_and_print_errs(
            generate_test_lockfile_with_past_zero_plan_key()
        )
        .is_ok());
    }
    #[test]
    fn test_plan_key_must_be_number() {
        assert!(validate_lock_file_json_and_print_errs(
            generate_test_lockfile_with_non_number_plan_key()
        )
        .is_err());
    }
    #[test]
    #[ignore] // failing
    fn test_valid_payment_conditions_only() {
        assert!(validate_lock_file_json_and_print_errs(
            generate_test_lockfile_with_invalid_plan_condition()
        )
        .is_err());
    }
    #[test]
    fn test_shares_cannot_be_negative() {
        assert!(validate_lock_file_json_and_print_errs(
            generate_test_lockfile_with_negative_shares()
        )
        .is_err());
    }
    #[test]
    #[ignore] // failing
    fn test_cannot_have_more_share_labels_than_payees() {
        assert!(validate_lock_file_json_and_print_errs(
            generate_test_lockfile_with_more_share_labels_than_payees()
        )
        .is_err());
    }
    #[test]
    #[ignore] // failing
    fn test_share_labels_must_match_payees() {
        assert!(validate_lock_file_json_and_print_errs(
            generate_test_lockfile_with_incorrect_share_label()
        )
        .is_err());
    }
}
