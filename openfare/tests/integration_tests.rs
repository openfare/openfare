use anyhow::Result;
use std::path::Path;
use std::process::Command;

const TEST_LOCK_FILE_NAME: &str = "test.lock";
const EXEC_PATH: &str = "../target/debug/openfare";

#[test]
fn test_lock_file_creation_and_validation_are_consistent() -> Result<()> {
    let tmp_dir = tempdir::TempDir::new("openfare_integration_test")?;
    let tmp_dir = tmp_dir.path().to_path_buf();

    assert!(Path::new(EXEC_PATH).exists());

    let lock_file_path = tmp_dir.join(TEST_LOCK_FILE_NAME);
    let lock_file_path = lock_file_path.to_str().unwrap();

    // create the lock file
    let output_status = Command::new(EXEC_PATH)
        .args(["lock", "new", "--lock-file-path", lock_file_path])
        .status()
        .expect("execution failed")
        .success();

    assert!(
        output_status,
        "Could not create new lock file {}",
        lock_file_path
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
            lock_file_path,
        ])
        .status()
        .expect("execution failed")
        .success();

    assert!(
        output_status,
        "Could not add plan to lock file {}",
        lock_file_path
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
            lock_file_path,
        ])
        .status()
        .expect("execution failed")
        .success();

    assert!(
        output_status,
        "Could not add plan to lock file {}",
        lock_file_path
    );

    // Validate the lock file
    let output_status = Command::new(EXEC_PATH)
        .args(["lock", "validate", "--lock-file-path", lock_file_path])
        .status()
        .expect("execution failed")
        .success();

    assert!(output_status, "Invalid lock file: {}", lock_file_path);
    Ok(())
}
