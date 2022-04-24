use crate::common::json::{Get, Set};
use anyhow::Result;
use structopt::{self, StructOpt};

mod common;
mod condition;
mod plan;
mod profile;
mod validate;

#[derive(Debug, Clone, StructOpt)]
pub struct Arguments {
    // SUBCOMMANDS
    #[structopt(subcommand)]
    commands: Subcommands,
}

#[derive(Debug, StructOpt, Clone)]
enum Subcommands {
    /// New lock file.
    New(NewArguments),
    /// Add plan, profile, etc.
    Add(AddArguments),
    /// Set lock field.
    Set(SetArguments),
    /// Remove plan, profile, condition, etc.
    Remove(RemoveSubcommands),
    /// Update payee profiles.
    Update(UpdateArguments),
    /// Show lock fields.
    Show(ShowArguments),
    /// Check if a lock file contains errors
    Validate(validate::Arguments),
}

pub fn run_command(args: &Arguments) -> Result<()> {
    match &args.commands {
        Subcommands::New(args) => {
            new(&args)?;
        }
        Subcommands::Add(args) => {
            add(&args)?;
        }
        Subcommands::Set(args) => {
            set(&args)?;
        }
        Subcommands::Remove(args) => {
            remove(&args)?;
        }
        Subcommands::Update(args) => {
            update(&args)?;
        }
        Subcommands::Show(args) => {
            show(&args)?;
        }
        Subcommands::Validate(args) => {
            validate::run_command(&args)?;
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
pub struct NewArguments {
    /// Overwrite existing file.
    #[structopt(long, short)]
    pub force: bool,

    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

fn new(args: &NewArguments) -> Result<()> {
    let lock_handle = crate::handles::LockHandle::new(&args.lock_file_args.path, args.force)?;
    println!("Created new lock file: {}", lock_handle.path().display());
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub enum AddArguments {
    /// Add plan.
    Plan(plan::AddArguments),

    /// Add payee profile to payment plan(s).
    Profile(profile::AddArguments),

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
        RemoveSubcommands::Condition(args) => {
            condition::remove(&args)?;
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub struct UpdateArguments {
    #[structopt(flatten)]
    pub profile: profile::UpdateArguments,
}

fn update(args: &UpdateArguments) -> Result<()> {
    profile::update(&args.profile)?;
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub struct SetArguments {
    /// Field path.
    #[structopt(name = "field-path")]
    pub path: String,

    /// Field value.
    pub value: String,
}

fn set(args: &SetArguments) -> Result<()> {
    let mut lock_handle = crate::handles::LockHandle::load(&None)?;
    lock_handle.set(&args.path, &args.value)?;
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
pub struct ShowArguments {
    /// Field path.
    #[structopt(name = "field-path")]
    pub path: Option<String>,
}

fn show(args: &ShowArguments) -> Result<()> {
    let lock_handle = crate::handles::LockHandle::load(&None)?;
    let value = lock_handle.get(&args.path)?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
