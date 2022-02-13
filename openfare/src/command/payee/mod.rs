use crate::common::config::FileStore;
use anyhow::Result;
use structopt::{self, StructOpt};

mod label;
mod payment_method;
mod url;
use crate::common;

#[derive(Debug, Clone, StructOpt)]
pub struct Arguments {
    // SUBCOMMANDS
    #[structopt(subcommand)]
    commands: Option<Subcommands>,
}

#[derive(Debug, StructOpt, Clone)]
enum Subcommands {
    /// Add payment method, label, URL, etc.
    Add(AddArguments),

    /// Remove payment method, label, URL, etc.
    Remove(RemoveArguments),
}

pub fn run_command(args: &Arguments) -> Result<()> {
    if let Some(subcommand) = &args.commands {
        match subcommand {
            Subcommands::Add(args) => {
                log::info!("Running command: payee add");
                add(&args)?;
            }
            Subcommands::Remove(args) => {
                log::info!("Running command: payee remove");
                remove(&args)?;
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

    /// Add label.
    Label(label::AddArguments),

    /// Add URL.
    Url(url::AddArguments),
}

fn add(args: &AddArguments) -> Result<()> {
    match &args {
        AddArguments::PaymentMethod(args) => {
            payment_method::add(&args)?;
        }
        AddArguments::Label(args) => {
            label::add(&args)?;
        }
        AddArguments::Url(args) => {
            url::add(&args)?;
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
enum RemoveArguments {
    /// Remove payment method.
    #[structopt(name = "payment-method")]
    PaymentMethod(payment_method::RemoveSubcommands),

    /// Remove label.
    Label(label::RemoveArguments),

    /// Remove URL.
    Url(url::RemoveArguments),
}

fn remove(args: &RemoveArguments) -> Result<()> {
    match args {
        RemoveArguments::PaymentMethod(args) => {
            payment_method::remove(&args)?;
        }
        RemoveArguments::Label(args) => {
            label::remove(&args)?;
        }
        RemoveArguments::Url(args) => {
            url::remove(&args)?;
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
    let payee = common::config::Payee::load()?;
    println!("{}", serde_json::to_string_pretty(&payee)?);
    Ok(())
}
