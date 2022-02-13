use anyhow::Result;
use structopt::{self, StructOpt};

mod common;
mod condition;
mod payment;
mod plan;
mod profile;

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
    /// New lock file.
    New(NewArguments),
    /// Add plan, profile, etc.
    Add(AddArguments),
    /// Remove plan, profile, payment, condition, etc.
    Remove(RemoveSubcommands),
}

pub fn run_command(args: &Arguments) -> Result<()> {
    if let Some(subcommand) = &args.commands {
        match subcommand {
            Subcommands::New(args) => {
                new(&args)?;
            }
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
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct NewArguments {
    /// Overwrite existing file.
    #[structopt(long, short)]
    pub force: bool,

    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

fn new(args: &NewArguments) -> Result<()> {
    let lock_handle = common::LockFileHandle::new(&args.lock_file_args.path, args.force)?;
    println!("Created new lock file: {}", lock_handle.path().display());
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub enum AddArguments {
    /// Add plan.
    Plan(plan::AddArguments),

    /// Add payee profile to payment plan(s).
    Profile(profile::AddArguments),

    /// Add payment parameters.
    Payment(payment::AddArguments),

    /// Add condition(s) to plan(s).
    Condition(condition::AddArguments),
}

fn add(args: &AddArguments) -> Result<()> {
    match &args {
        AddArguments::Plan(args) => {
            plan::add(&args)?;
        }
        AddArguments::Profile(args) => {
            profile::add(&args)?;
        }
        AddArguments::Payment(args) => {
            payment::add(&args)?;
        }
        AddArguments::Condition(args) => {
            condition::add(&args)?;
        }
    }

    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub enum RemoveSubcommands {
    /// Remove plan.
    Plan(plan::RemoveArguments),

    /// Remove payee profile from payment plan(s).
    Profile(profile::RemoveArguments),

    /// Remove payment parameters from payment plan(s).
    Payment(payment::RemoveArguments),

    /// Remove condition(s) from payment plan(s).
    Condition(condition::RemoveArguments),
}

fn remove(subcommand: &RemoveSubcommands) -> Result<()> {
    match subcommand {
        RemoveSubcommands::Plan(args) => {
            plan::remove(&args)?;
        }
        RemoveSubcommands::Profile(args) => {
            profile::remove(&args)?;
        }
        RemoveSubcommands::Payment(args) => {
            payment::remove(&args)?;
        }
        RemoveSubcommands::Condition(args) => {
            condition::remove(&args)?;
        }
    }
    Ok(())
}

fn show(_verbosity: u8) -> Result<()> {
    let lock_handle = common::LockFileHandle::load(&None)?;
    println!("{}", serde_json::to_string_pretty(&lock_handle.lock)?);
    Ok(())
}
