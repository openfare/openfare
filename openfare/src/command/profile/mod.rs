use crate::common::config::FileStore;
use anyhow::Result;
use structopt::{self, StructOpt};

mod payment_method;
mod push;

#[derive(Debug, Clone, StructOpt)]
pub struct Arguments {
    // SUBCOMMANDS
    #[structopt(subcommand)]
    commands: Option<Subcommands>,
}

#[derive(Debug, StructOpt, Clone)]
enum Subcommands {
    /// Add payment method, etc.
    Add(AddArguments),

    /// Set payment method fields, etc.
    Set(SetArguments),

    /// Remove payment method, etc.
    Remove(RemoveArguments),

    /// Push profile to git repository URL.
    Push(push::Arguments),
}

pub fn run_command(args: &Arguments) -> Result<()> {
    if let Some(subcommand) = &args.commands {
        match subcommand {
            Subcommands::Add(args) => {
                log::info!("Running command: profile add");
                add(&args)?;
            }
            Subcommands::Set(args) => {
                log::info!("Running command: profile set");
                set(&args)?;
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
    } else {
        show()?;
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
    pub path: String,

    /// Field value.
    pub value: String,
}

fn set(args: &SetArguments) -> Result<()> {
    let mut profile = crate::profile::Profile::load()?;
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
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct ShowArguments {}

fn show() -> Result<()> {
    let profile = crate::profile::Profile::load()?;
    println!("{}", serde_json::to_string_pretty(&profile)?);
    Ok(())
}
