use anyhow::Result;
use structopt::{self, StructOpt};

use crate::common::config::FileStore;

#[derive(Debug, StructOpt, Clone)]
pub struct AddArguments {
    /// Payee label.
    label: String,
}

pub fn add(args: &AddArguments) -> Result<()> {
    let mut payee = crate::common::config::Payee::load()?;
    (*payee).label = args.label.clone();
    payee.dump()?;
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub struct RemoveArguments {}

pub fn remove(_args: &RemoveArguments) -> Result<()> {
    let mut payee = crate::common::config::Payee::load()?;
    (*payee).label = "".to_string();
    payee.dump()?;
    Ok(())
}
