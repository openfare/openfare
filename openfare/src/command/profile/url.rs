use anyhow::Result;
use structopt::{self, StructOpt};

use crate::common::config::FileStore;

#[derive(Debug, StructOpt, Clone)]
pub struct AddArguments {
    /// Profile URL.
    url: String,
}

pub fn add(args: &AddArguments) -> Result<()> {
    let mut profile = crate::profile::Profile::load()?;
    (*profile).url = args.url.clone();
    profile.dump()?;
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub struct RemoveArguments {}

pub fn remove(_args: &RemoveArguments) -> Result<()> {
    let mut profile = crate::profile::Profile::load()?;
    (*profile).url = "".to_string();
    profile.dump()?;
    Ok(())
}
