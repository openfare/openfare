use crate::common::config::FileStore;
use anyhow::Result;
use structopt::{self, StructOpt};
mod btc_lightning_keysend;
mod paypal;

#[derive(Debug, Clone, StructOpt)]
pub struct Arguments {
    #[structopt(name = "verbosity", short, long, parse(from_occurrences))]
    verbosity: u8,

    // SUBCOMMANDS
    #[structopt(subcommand)]
    commands: Option<Subcommands>,
}

#[derive(Debug, StructOpt, Clone)]
enum Subcommands {
    /// Add payment method.
    Add(AddSubcommands),

    /// Remove payment method.
    Remove(RemoveSubcommands),
}

pub fn run_command(args: &Arguments) -> Result<()> {
    if let Some(subcommand) = &args.commands {
        match subcommand {
            Subcommands::Add(args) => {
                add(&args)?;
            }
            Subcommands::Remove(args) => {
                remove(&args)?;
            }
        }
    } else {
        show(args.verbosity)?;
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub enum AddSubcommands {
    /// Set PayPal payment method.
    #[structopt(name = "paypal")]
    PayPal(paypal::AddArguments),

    /// Set BTC lightning keysend payment method.
    #[structopt(name = "btc-lightning-keysend")]
    BtcLightningKeysend(btc_lightning_keysend::AddArguments),
}

fn add(subcommand: &AddSubcommands) -> Result<()> {
    match subcommand {
        AddSubcommands::PayPal(args) => {
            paypal::add(&args)?;
        }
        AddSubcommands::BtcLightningKeysend(args) => {
            btc_lightning_keysend::add(&args)?;
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub enum RemoveSubcommands {
    /// Remove PayPal payment method.
    #[structopt(name = "paypal")]
    PayPal(paypal::RemoveArguments),

    /// Remove BTC lightning keysend payment method.
    #[structopt(name = "btc-lightning-keysend")]
    BtcLightningKeysend(btc_lightning_keysend::RemoveArguments),
}

fn remove(subcommand: &RemoveSubcommands) -> Result<()> {
    match subcommand {
        RemoveSubcommands::PayPal(args) => {
            paypal::remove(&args)?;
        }
        RemoveSubcommands::BtcLightningKeysend(args) => {
            btc_lightning_keysend::remove(&args)?;
        }
    }
    Ok(())
}

fn show(verbosity: u8) -> Result<()> {
    let payees = crate::common::config::Payees::load()?;
    if let Some((_name, payee)) = payees.active()? {
        let methods = payee.payment_methods()?;
        if verbosity == 0 {
            show_method_names_only(&methods);
        } else if verbosity >= 1 {
            show_method_details(&methods)?;
        }
    }
    Ok(())
}

fn show_method_names_only(
    methods: &Vec<Box<dyn openfare_lib::lock::payee::payment_methods::PaymentMethod>>,
) {
    let names = methods.iter().map(|m| m.name()).collect::<Vec<String>>();
    println!("{}", names.join("\n"))
}

fn show_method_details(
    methods: &Vec<Box<dyn openfare_lib::lock::payee::payment_methods::PaymentMethod>>,
) -> Result<()> {
    let mut json_methods = vec![];
    for method in methods {
        let method = maplit::btreemap! {
            method.name() => method.to_serde_json_value()?
        };
        json_methods.push(method);
    }
    println!("{}", serde_json::to_string_pretty(&json_methods)?);
    Ok(())
}
