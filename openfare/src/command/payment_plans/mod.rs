use anyhow::{format_err, Result};
use std::convert::TryFrom;
use structopt::{self, StructOpt};

mod fs;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct Arguments {
    /// Force setup cleanly. Removes existing local setup data.
    #[structopt(long = "force", short = "f")]
    pub force: bool,
}

pub fn run_command(args: &Arguments) -> Result<()> {
    fs::setup(args.force)?;
    Ok(())
}

/// Return Err if setup is not complete, otherwise Result.
pub fn is_complete() -> Result<()> {
    if !fs::is_complete()? {
        return Err(format_err!(
            "Setup command has not been executed. Try running: 'openfare setup --help'"
        ));
    }
    Ok(())
}
