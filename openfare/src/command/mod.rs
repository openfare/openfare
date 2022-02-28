use anyhow::Result;
use structopt::{self, StructOpt};

mod config;
mod extensions;
mod lock;
mod pay;
mod price;
mod profile;
mod service;

#[derive(Debug, StructOpt, Clone)]
pub enum Command {
    /// Price a package and its dependencies.
    Price(price::Arguments),

    /// Pay fees or donations to project dependencies.
    Pay(pay::Arguments),

    /// Manage profile.
    Profile(profile::Arguments),

    /// Manage lock file.
    Lock(lock::Arguments),

    /// Manage payment services.
    Service(service::Arguments),

    /// Configure settings.
    Config(config::Arguments),

    /// Manage extensions.
    Extensions(extensions::Arguments),
}

pub fn run_command(command: Command, extension_args: &Vec<String>) -> Result<()> {
    crate::setup::ensure()?;
    log::info!("Running command: {:?}", command);
    match command {
        Command::Price(args) => {
            price::run_command(&args, &extension_args)?;
        }
        Command::Pay(args) => {
            pay::run_command(&args, &extension_args)?;
        }
        Command::Profile(args) => {
            profile::run_command(&args)?;
        }
        Command::Lock(args) => {
            lock::run_command(&args)?;
        }
        Command::Service(args) => {
            service::run_command(&args)?;
        }
        Command::Config(args) => {
            config::run_command(&args)?;
        }
        Command::Extensions(args) => {
            extensions::run_command(&args)?;
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "Micropayment funded software.")]
#[structopt(global_setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(global_setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
pub struct Opts {
    #[structopt(subcommand)]
    pub command: Command,
}
