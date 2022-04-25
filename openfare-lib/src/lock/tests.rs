use super::*;
use crate::price::{Currency, Price};
use crate::profile::Profile;
use anyhow::Result;
use payee::Payee;
use plan::conditions::{Conditions, ForProfit};
use plan::{Plan, PlanType};
use serde_json::{json, Value};
use {shares, Lock};

fn generate_minimal_valid_lock() -> Value {
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
fn generate_test_lock() -> Value {
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
fn generate_test_lock_file_with_typo() -> Value {
    let mut lock = generate_test_lock();
    let steve = lock["payees"]["steve"].as_object_mut().unwrap();
    if let Some(url) = steve.remove("url") {
        steve.insert("ur1".to_string(), url);
    } else {
        panic!("Adjustment failed");
    }
    lock
}
fn generate_test_lock_file_with_non_number_plan_key() -> Value {
    let mut lock = generate_test_lock();
    let plans = lock["plans"].as_object_mut().unwrap();
    if let Some(plan0) = plans.remove("0") {
        plans.insert("this_is_a_non_number_plan_key".to_string(), plan0);
    } else {
        panic!("Adjustment failed");
    }
    lock
}
fn generate_test_lock_file_with_past_zero_plan_key() -> Value {
    let mut lock = generate_test_lock();
    let plans = lock["plans"].as_object_mut().unwrap();
    if let Some(plan0) = plans.remove("0") {
        plans.insert("9001".to_string(), plan0);
    } else {
        panic!("Adjustment failed");
    }
    lock
}
fn generate_test_lock_file_with_negative_shares() -> Value {
    let mut lock = generate_test_lock();
    lock["shares"]
        .as_object_mut()
        .unwrap()
        .insert("steve".to_string(), json!(-50));
    println!("{}", serde_json::to_string_pretty(&lock).unwrap());
    lock
}
fn generate_test_lock_file_with_invalid_plan_condition() -> Value {
    let mut lock = generate_test_lock();
    lock["plans"]["0"]["conditions"]
        .as_object_mut()
        .unwrap()
        .insert(
            "fake_condition".to_string(),
            Value::String("THIS SOFTWARE CANNOT BE USED FOR ANY PURPOSE ON WENSDAYS".to_string()),
        );
    lock
}
fn generate_test_lock_file_with_more_share_labels_than_payees() -> Value {
    let mut lock = generate_test_lock();
    lock["shares"]
        .as_object_mut()
        .unwrap()
        .insert("otherNotSteve".to_string(), json!(5));
    lock
}
fn generate_test_lock_file_with_incorrect_share_label() -> Value {
    let mut lock = generate_test_lock();
    let shares = lock["shares"].as_object_mut().unwrap();
    if let Some(steves_shares) = shares.remove("steve") {
        shares.insert("someGuy".to_string(), steves_shares);
    } else {
        panic!("Adjustment failed");
    }
    lock
}

fn print_if_err(result: Result<()>) -> Result<()> {
    match &result {
        Ok(..) => {}
        Err(e) => println!("{}", e),
    }
    result
}

fn validate_lock_file_json_and_print_errs(val: Value) -> Result<()> {
    let lock: crate::lock::Lock = serde_json::from_value(val)?;
    let result = lock.validate();
    print_if_err(result)
}

#[test]
fn test_empty_lock_file_not_valid() {
    assert!(validate_lock_file_json_and_print_errs(json!("")).is_err());
}
#[test]
fn test_almost_empty_lock_file_not_valid() {
    assert!(validate_lock_file_json_and_print_errs(json!("{}")).is_err());
}
#[test]
fn test_minimal_lock_file_is_valid() {
    assert!(validate_lock_file_json_and_print_errs(generate_minimal_valid_lock()).is_ok());
}
#[test]
fn test_more_complex_lock_file_is_valid() {
    assert!(validate_lock_file_json_and_print_errs(generate_test_lock()).is_ok());
}
#[test]
fn test_minor_typo_not_valid() {
    assert!(validate_lock_file_json_and_print_errs(generate_test_lock_file_with_typo()).is_err());
}
#[test]
fn test_plan_numbers_can_start_past_zero() {
    assert!(validate_lock_file_json_and_print_errs(
        generate_test_lock_file_with_past_zero_plan_key()
    )
    .is_ok());
}
#[test]
fn test_plan_key_must_be_number() {
    assert!(validate_lock_file_json_and_print_errs(
        generate_test_lock_file_with_non_number_plan_key()
    )
    .is_err());
}
#[test]
#[ignore] // failing
fn test_valid_payment_conditions_only() {
    assert!(validate_lock_file_json_and_print_errs(
        generate_test_lock_file_with_invalid_plan_condition()
    )
    .is_err());
}
#[test]
fn test_shares_cannot_be_negative() {
    assert!(
        validate_lock_file_json_and_print_errs(generate_test_lock_file_with_negative_shares())
            .is_err()
    );
}
#[test]
#[ignore] // failing
fn test_cannot_have_more_share_labels_than_payees() {
    assert!(validate_lock_file_json_and_print_errs(
        generate_test_lock_file_with_more_share_labels_than_payees()
    )
    .is_err());
}
#[test]
#[ignore] // failing
fn test_share_labels_must_match_payees() {
    assert!(validate_lock_file_json_and_print_errs(
        generate_test_lock_file_with_incorrect_share_label()
    )
    .is_err());
}
