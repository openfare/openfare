use super::common;
use anyhow::Result;
use structopt::{self, StructOpt};

#[derive(Debug, StructOpt, Clone)]
pub struct Arguments {
    /// Optional path to lock file.
    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

pub fn run_command(args: &Arguments) -> Result<()> {
    let lock_handle = crate::handles::LockHandle::load(&args.lock_file_args.path)?;
    match lock_handle.lock.validate() {
        Ok(_) => {
            println!("Lock file is valid.")
        }
        Err(e) => {
            println!("Error validating lock file: {}", e)
        }
    };
    Ok(())
}
