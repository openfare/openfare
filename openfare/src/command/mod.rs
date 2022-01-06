use anyhow::Result;
use structopt::{self, StructOpt};

mod config;
mod extension;
mod payee;
mod price;

pub fn run_command(command: Command, extension_args: &Vec<String>) -> Result<()> {
    match command {
        Command::Price(args) => {
            log::info!("Running command: price");
            crate::setup::ensure()?;
            price::run_command(&args, &extension_args)?;
        }
        Command::Payee(args) => {
            log::info!("Running command: payee");
            crate::setup::ensure()?;
            payee::run_command(&args)?;
        }
        Command::Config(args) => {
            log::info!("Running command: config");
            crate::setup::ensure()?;
            config::run_command(&args)?;
        }
        Command::Extension(args) => {
            log::info!("Running command: extension");
            crate::setup::ensure()?;
            extension::run_subcommand(&args)?;
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
    Payee(payee::Subcommands),

    /// Configure settings.
    #[structopt(name = "config")]
    Config(config::Arguments),

    /// Manage extensions.
    #[structopt(name = "extension")]
    Extension(extension::Subcommands),
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "Monetize software with one commit.")]
#[structopt(global_setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(global_setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
pub struct Opts {
    #[structopt(subcommand)]
    pub command: Command,
}
