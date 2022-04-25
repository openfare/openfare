use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;

const TEST_LOCK_FILE_FOLDER: &str = "/tmp/openfare-test-folder/";
const TEST_LOCK_FILE_NAME: &str = "test.lock";
const EXEC_PATH: &str = "../target/debug/openfare";

fn temp_folder_setup() -> Result<()> {
    fs::remove_dir_all(TEST_LOCK_FILE_FOLDER).ok();
    fs::create_dir(TEST_LOCK_FILE_FOLDER)?;
    Ok(())
}

#[test]
fn test_lock_file_creation_and_validation_are_consistent() {
    temp_folder_setup().expect(&format!(
        "Problem creating test folder at {}",
        TEST_LOCK_FILE_FOLDER
    ));

    assert!(Path::new(EXEC_PATH).exists());

    let test_lock_file_full_path = &([TEST_LOCK_FILE_FOLDER, TEST_LOCK_FILE_NAME].concat());

    // create the lock file
    let output_status = Command::new(EXEC_PATH)
        .args(["lock", "new", "--lock-file-path", test_lock_file_full_path])
        .status()
        .expect("execution failed")
        .success();

    assert!(
        output_status,
        "Could not create new lock file {}",
        test_lock_file_full_path
    );

    // Add a plan
    let output_status = Command::new(EXEC_PATH)
        .args([
            "lock",
            "add",
            "plan",
            "compulsory",
            "--price",
            "5usd",
            "--lock-file-path",
            test_lock_file_full_path,
        ])
        .status()
        .expect("execution failed")
        .success();

    assert!(
        output_status,
        "Could not add plan to lock file {}",
        test_lock_file_full_path
    );

    // Add a payee
    let output_status = Command::new(EXEC_PATH)
        .args([
            "lock",
            "add",
            "profile",
            "--shares",
            "500",
            "--label",
            "steve",
            "--lock-file-path",
            test_lock_file_full_path,
        ])
        .status()
        .expect("execution failed")
        .success();

    assert!(
        output_status,
        "Could not add plan to lock file {}",
        test_lock_file_full_path
    );

    // Validate the lock file
    let output_status = Command::new(EXEC_PATH)
        .args([
            "lock",
            "validate",
            "--lock-file-path",
            test_lock_file_full_path,
        ])
        .status()
        .expect("execution failed")
        .success();

    assert!(
        output_status,
        "Invalid lock file: {}",
        test_lock_file_full_path
    );
}
