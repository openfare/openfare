use crate::common::fs::FileStore;
use crate::common::json::{Get, Set};
use anyhow::Result;
use structopt::{self, StructOpt};

mod lnpay;

#[derive(Debug, Clone, StructOpt)]
pub struct Arguments {
    // SUBCOMMANDS
    #[structopt(subcommand)]
    commands: Subcommands,
}

#[derive(Debug, StructOpt, Clone)]
enum Subcommands {
    /// Add payment service.
    Add(AddArguments),

    /// Set payment service parameters.
    Set(SetArguments),

    /// Show payment service parameters.
    Show(ShowArguments),

    /// Remove payment service.
    Remove(RemoveArguments),
}

pub fn run_command(args: &Arguments) -> Result<()> {
    match &args.commands {
        Subcommands::Add(args) => {
            log::info!("Running command: service add");
            add(&args)?;
        }
        Subcommands::Set(args) => {
            log::info!("Running command: service set");
            set(&args)?;
        }
        Subcommands::Show(args) => {
            log::info!("Running command: service show");
            show(&args)?;
        }
        Subcommands::Remove(args) => {
            log::info!("Running command: service remove");
            remove(&args)?;
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub enum AddArguments {
    /// Add service LNPAY (https://lnpay.co)
    #[structopt(name = "lnpay")]
    LnPay(lnpay::AddArguments),
}

fn add(args: &AddArguments) -> Result<()> {
    match &args {
        AddArguments::LnPay(args) => {
            lnpay::add(&args)?;
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
    config.services.set(&args.path, &args.value)?;
    config.dump()?;
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
enum RemoveArguments {
    /// Add service LNPAY (https://lnpay.co)
    #[structopt(name = "lnpay")]
    LnPay(lnpay::RemoveArguments),
}

fn remove(args: &RemoveArguments) -> Result<()> {
    match args {
        RemoveArguments::LnPay(args) => {
            lnpay::remove(&args)?;
        }
    }
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
    let value = config.services.get(&args.path)?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
