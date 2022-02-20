use crate::common::config::FileStore;
use crate::common::json::{Get, Set};
use anyhow::Result;
use structopt::{self, StructOpt};

mod payment_method;
mod push;

#[derive(Debug, Clone, StructOpt)]
pub struct Arguments {
    // SUBCOMMANDS
    #[structopt(subcommand)]
    commands: Subcommands,
}

#[derive(Debug, StructOpt, Clone)]
enum Subcommands {
    /// Add payment method, etc.
    Add(AddArguments),

    /// Set payment method fields, etc.
    Set(SetArguments),

    /// Show profile fields.
    Show(ShowArguments),

    /// Remove payment method, etc.
    Remove(RemoveArguments),

    /// Push profile to git repository URL.
    Push(push::Arguments),
}

pub fn run_command(args: &Arguments) -> Result<()> {
    match &args.commands {
        Subcommands::Add(args) => {
            log::info!("Running command: profile add");
            add(&args)?;
        }
        Subcommands::Set(args) => {
            log::info!("Running command: profile set");
            set(&args)?;
        }
        Subcommands::Show(args) => {
            log::info!("Running command: profile show");
            show(&args)?;
        }
        Subcommands::Remove(args) => {
            log::info!("Running command: profile remove");
            remove(&args)?;
        }
        Subcommands::Push(args) => {
            log::info!("Running command: profile push");
            push::push(&args)?;
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub enum AddArguments {
    /// Add payment method.
    #[structopt(name = "payment-method")]
    PaymentMethod(payment_method::AddSubcommands),
}

fn add(args: &AddArguments) -> Result<()> {
    match &args {
        AddArguments::PaymentMethod(args) => {
            payment_method::add(&args)?;
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
    let mut profile = crate::handles::ProfileHandle::load()?;
    profile.set(&args.path, &args.value)?;
    profile.dump()?;
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
enum RemoveArguments {
    /// Remove payment method.
    #[structopt(name = "payment-method")]
    PaymentMethod(payment_method::RemoveSubcommands),
}

fn remove(args: &RemoveArguments) -> Result<()> {
    match args {
        RemoveArguments::PaymentMethod(args) => {
            payment_method::remove(&args)?;
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
    let profile = crate::handles::ProfileHandle::load()?;
    let value = profile.get(&args.path)?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
