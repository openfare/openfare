use anyhow::Result;
use structopt::{self, StructOpt};
mod btc_lightning;
mod paypal;

#[derive(Debug, StructOpt, Clone)]
pub enum AddSubcommands {
    /// Set payment method: PayPal.
    #[structopt(name = "paypal")]
    PayPal(paypal::AddArguments),

    /// Set payment method: Bitcoin Lightning Network.
    #[structopt(name = "btc-ln")]
    BtcLightning(btc_lightning::AddArguments),
}

pub fn add(subcommand: &AddSubcommands) -> Result<()> {
    match subcommand {
        AddSubcommands::PayPal(args) => {
            paypal::add(&args)?;
        }
        AddSubcommands::BtcLightning(args) => {
            btc_lightning::add(&args)?;
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub enum RemoveSubcommands {
    /// Remove payment method: PayPal.
    #[structopt(name = "paypal")]
    PayPal(paypal::RemoveArguments),

    /// Remove payment method: Bitcoin Lightning Network.
    #[structopt(name = "btc-ln")]
    BtcLightning(btc_lightning::RemoveArguments),
}

pub fn remove(subcommand: &RemoveSubcommands) -> Result<()> {
    match subcommand {
        RemoveSubcommands::PayPal(args) => {
            paypal::remove(&args)?;
        }
        RemoveSubcommands::BtcLightning(args) => {
            btc_lightning::remove(&args)?;
        }
    }
    Ok(())
}
