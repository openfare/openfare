use anyhow::Result;
use structopt::{self, StructOpt};

mod config;
mod extension;
mod lock;
mod payee;
mod payment_method;
mod price;

pub fn run_command(command: Command, extension_args: &Vec<String>) -> Result<()> {
    crate::setup::ensure()?;
    log::info!("Running command: {:?}", command);
    match command {
        Command::Price(args) => {
            price::run_command(&args, &extension_args)?;
        }
        Command::Payee(args) => {
            payee::run_command(&args)?;
        }
        Command::PaymentMethod(args) => {
            payment_method::run_command(&args)?;
        }
        Command::Lock(args) => {
            lock::run_command(&args)?;
        }
        Command::Config(args) => {
            config::run_command(&args)?;
        }
        Command::Extension(args) => {
            extension::run_command(&args)?;
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub enum Command {
    /// Price a package and its dependencies.
    #[structopt(name = "price")]
    Price(price::Arguments),

    /// Manage payee profiles.
    #[structopt(name = "payee")]
    Payee(payee::Arguments),

    /// Manage payee payment methods.
    #[structopt(name = "payment-method")]
    PaymentMethod(payment_method::Arguments),

    /// Manage lock file.
    #[structopt(name = "lock")]
    Lock(lock::Arguments),

    /// Configure settings.
    #[structopt(name = "config")]
    Config(config::Arguments),

    /// Manage extensions.
    #[structopt(name = "extension")]
    Extension(extension::Arguments),
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "Monetize software with one commit.")]
#[structopt(global_setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(global_setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
pub struct Opts {
    #[structopt(subcommand)]
    pub command: Command,
}
