use anyhow::Result;
use structopt::{self, StructOpt};
mod btc_lightning_keysend;
mod paypal;

#[derive(Debug, StructOpt, Clone)]
pub enum AddSubcommands {
    /// Set PayPal payment method.
    PayPal(paypal::AddArguments),

    /// Set BTC lightning keysend payment method.
    #[structopt(name = "btc-lightning-keysend")]
    BtcLightningKeysend(btc_lightning_keysend::AddArguments),
}

pub fn add(subcommand: &AddSubcommands) -> Result<()> {
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

pub fn remove(subcommand: &RemoveSubcommands) -> Result<()> {
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
