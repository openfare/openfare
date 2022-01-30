use crate::common::config::FileStore;
use anyhow::Result;
use structopt::{self, StructOpt};

use crate::common;

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
    /// Add new payee.
    Add(AddArguments),

    /// Set active payee.
    Activate(ActivateArguments),

    /// Rename payee.
    Rename(RenameArguments),

    /// Remove payee.
    Remove(RemoveArguments),
}

pub fn run_command(args: &Arguments) -> Result<()> {
    if let Some(subcommand) = &args.commands {
        match subcommand {
            Subcommands::Add(args) => {
                log::info!("Running command: payee add");
                add(&args)?;
            }
            Subcommands::Activate(args) => {
                log::info!("Running command: payee activate");
                activate(&args)?;
            }
            Subcommands::Rename(args) => {
                log::info!("Running command: payee rename");
                rename(&args)?;
            }
            Subcommands::Remove(args) => {
                log::info!("Running command: payee remove");
                remove(&args)?;
            }
        }
    } else {
        show(&args.verbosity)?;
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct AddArguments {
    /// Payee name label.
    pub name: String,

    /// Skip setting active profile to new payee profile.
    #[structopt(long = "skip-activate")]
    pub skip_activate: bool,
}

fn add(args: &AddArguments) -> Result<()> {
    log::debug!("Adding new payee profile.");
    let mut payees = common::config::Payees::load()?;
    payees.add(&args.name)?;

    if !args.skip_activate {
        log::debug!("Setting new profile to active.");
        payees.activate(&args.name)?;
    } else {
        log::debug!("Not setting new profile to active.");
    }

    payees.dump()?;
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct ActivateArguments {
    /// Payee name label.
    pub name: String,
}

fn activate(args: &ActivateArguments) -> Result<()> {
    let mut payees = common::config::Payees::load()?;
    payees.activate(&args.name)?;
    payees.dump()?;
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct ShowArguments {}

fn show(verbosity: &u8) -> Result<()> {
    let payees = common::config::Payees::load()?;
    if *verbosity > 0 {
        println!("{}", serde_json::to_string_pretty(&payees)?);
    } else {
        let active_payee = if let Some((active_payee, _)) = payees.active()? {
            Some(active_payee)
        } else {
            None
        };
        for (name, _payee) in payees.payees().iter() {
            let active_status_tag = if let Some(active_payee) = active_payee {
                if name == active_payee {
                    "(active)"
                } else {
                    ""
                }
            } else {
                ""
            };
            println!("{name} {active}", name = name, active = active_status_tag);
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
pub struct RenameArguments {
    /// Old payee name.
    #[structopt(name = "old-name")]
    pub old_name: String,

    /// New payee name.
    #[structopt(name = "new-name")]
    pub new_name: String,
}

fn rename(args: &RenameArguments) -> Result<()> {
    let mut payees = common::config::Payees::load()?;
    payees.rename(&args.old_name, &args.new_name)?;
    payees.dump()?;
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct RemoveArguments {
    /// Payee name label.
    pub name: String,
}

fn remove(args: &RemoveArguments) -> Result<()> {
    let mut payees = common::config::Payees::load()?;
    payees.remove(&args.name)?;
    payees.dump()?;
    Ok(())
}
