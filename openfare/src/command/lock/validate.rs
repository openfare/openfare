use super::common;
use crate::handles::lock;
use anyhow::Result;
use serde_json::Value;
use structopt::{self, StructOpt};

#[derive(Debug, StructOpt, Clone)]
pub struct Arguments {
    /// Optional path to lock file.
    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

pub fn run_command(args: &Arguments) -> Result<()> {
    let result = validate_lock_file(&args.lock_file_args);
    if result.is_ok() {
        println!("Lockfile is valid! ✅");
    }
    result
}

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
