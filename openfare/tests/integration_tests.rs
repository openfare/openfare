use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;

const TEST_LOCKFILE_FOLDER: &str = "/tmp/openfare-test-folder/";
const TEST_LOCKFILE_NAME: &str = "test.lock";
const EXEC_PATH: &str = "../target/debug/openfare";

fn temp_folder_setup() -> Result<()> {
    fs::remove_dir_all(TEST_LOCKFILE_FOLDER).ok();
    fs::create_dir(TEST_LOCKFILE_FOLDER)?;
    Ok(())
}

#[test]
fn test_lockfile_creation_and_validation_are_consistent() {
    temp_folder_setup().expect(&format!(
        "Problem creating test folder at {}",
        TEST_LOCKFILE_FOLDER
    ));

    assert!(Path::new(EXEC_PATH).exists());

    let test_lockfile_full_path = &([TEST_LOCKFILE_FOLDER, TEST_LOCKFILE_NAME].concat());

    // create the lock file
    let output_status = Command::new(EXEC_PATH)
        .args(["lock", "new", "--lock-file-path", test_lockfile_full_path])
        .status()
        .expect("execution failed")
        .success();

    assert!(
        output_status,
        "Could not create new lockfile at {}",
        test_lockfile_full_path
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
            test_lockfile_full_path,
        ])
        .status()
        .expect("execution failed")
        .success();

    assert!(
        output_status,
        "Could not add plan to lockfile at {}",
        test_lockfile_full_path
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
            test_lockfile_full_path,
        ])
        .status()
        .expect("execution failed")
        .success();

    assert!(
        output_status,
        "Could not add plan to lockfile at {}",
        test_lockfile_full_path
    );

    // Validate the lock file
    let output_status = Command::new(EXEC_PATH)
        .args([
            "lock",
            "validate",
            "--lock-file-path",
            test_lockfile_full_path,
        ])
        .status()
        .expect("execution failed")
        .success();

    assert!(
        output_status,
        "Lockfile at {} not valid 😔",
        test_lockfile_full_path
    );
}
