use crate::common::fs::FileStore;
use crate::common::json::{Get, Set};
use anyhow::Result;
use structopt::{self, StructOpt};

#[derive(Debug, Clone, StructOpt)]
pub struct Arguments {
    // SUBCOMMANDS
    #[structopt(subcommand)]
    commands: Subcommands,
}

#[derive(Debug, StructOpt, Clone)]
enum Subcommands {
    /// Set config field values.
    Set(SetArguments),

    /// Show config field values.
    Show(ShowArguments),
}

pub fn run_command(args: &Arguments) -> Result<()> {
    match &args.commands {
        Subcommands::Set(args) => {
            log::info!("Running command: config set");
            set(&args)?;
        }
        Subcommands::Show(args) => {
            log::info!("Running command: config show");
            show(&args)?;
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub struct SetArguments {
    /// Field path.
    #[structopt(name = "field-path")]
    pub path: String,

    /// Field value.
    pub value: String,
}

fn set(args: &SetArguments) -> Result<()> {
    let mut config = crate::config::Config::load()?;
    config.set(&args.path, &args.value)?;
    config.dump()?;
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub struct ShowArguments {
    /// Field path.
    #[structopt(name = "field-path")]
    pub path: Option<String>,
}

fn show(args: &ShowArguments) -> Result<()> {
    let config = crate::config::Config::load()?;
    let value = config.get(&args.path)?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
