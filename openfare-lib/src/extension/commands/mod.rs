use super::common::Extension;
use anyhow::Result;
use structopt::{self, StructOpt};

mod common;
pub mod fs_defined_dependencies_configs;
pub mod package_dependencies_configs;
mod static_data;

#[derive(Debug, StructOpt, Clone)]
enum Command {
    /// Get extension static data.
    #[structopt(name = "static-data")]
    StaticData,

    /// Identify package dependencies.
    #[structopt(name = package_dependencies_configs::COMMAND_NAME)]
    PackageDependenciesConfigs(package_dependencies_configs::Arguments),

    /// Identify file defined dependencies.
    #[structopt(name = fs_defined_dependencies_configs::COMMAND_NAME)]
    FsDefinedDependenciesConfigs(fs_defined_dependencies_configs::Arguments),
}

fn run_command<T: Extension + std::fmt::Debug>(command: Command, extension: &mut T) -> Result<()> {
    match command {
        Command::StaticData => {
            static_data::run_command(extension)?;
        }

        Command::PackageDependenciesConfigs(args) => {
            package_dependencies_configs::run_command(&args, extension)?;
        }

        Command::FsDefinedDependenciesConfigs(args) => {
            fs_defined_dependencies_configs::run_command(&args, extension)?;
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "Package Code Reviews")]
#[structopt(global_setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(global_setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
struct Opts {
    #[structopt(subcommand)]
    pub command: Command,
}

pub fn run<T: Extension + std::fmt::Debug>(extension: &mut T) -> Result<()> {
    let commands = Opts::from_args();
    match run_command(commands.command, extension) {
        Ok(_) => {}
        Err(_e) => std::process::exit(-2),
    };
    Ok(())
}
