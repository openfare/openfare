use crate::common::fs::FileStore;
use anyhow::Result;
use structopt::{self, StructOpt};

use crate::extensions;

mod common;
mod format;
mod package;
mod project;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct Arguments {
    /// Package name.
    #[structopt(name = "package-name")]
    pub package_name: Option<String>,

    /// Package version.
    #[structopt(name = "package-version", requires("package-name"))]
    pub package_version: Option<String>,

    /// Specify an extension for handling the package and its dependencies.
    /// Example values: py, js, rs
    #[structopt(long = "extension", short = "e", name = "name")]
    pub extension_names: Option<Vec<String>>,
}

pub fn run_command(args: &Arguments, extension_args: &Vec<String>) -> Result<()> {
    let mut config = crate::config::Config::load()?;
    extensions::manage::update_config(&mut config)?;
    let extensions = extensions::manage::from_names_arg(&args.extension_names, &config)?;

    match &args.package_name {
        Some(package_name) => {
            let extensions_results = package::query_extensions(
                &package_name,
                &args.package_version.as_deref(),
                &extensions,
                &extension_args,
            )?;
            for (_extension, result) in extensions_results {
                check_conditions_parameters_set(&result.package_locks, &mut config)?;
            }
            package::price(
                &package_name,
                &args.package_version.as_deref(),
                &extensions,
                &extension_args,
                &config,
            )?;
        }
        None => {
            let extensions_results = project::query_extensions(&extensions, &extension_args)?;
            for (_extension, result) in extensions_results {
                check_conditions_parameters_set(&result.package_locks, &mut config)?;
            }
            project::price(&extensions, &extension_args, &config)?;
        }
    }
    Ok(())
}

/// Check correct parameters set for the given package locks conditions.
fn check_conditions_parameters_set(
    package_locks: &openfare_lib::package::PackageLocks,
    config: &mut crate::config::Config,
) -> Result<()> {
    let mut parameter_modified = false;
    for metadata in package_locks.conditions_metadata() {
        log::info!("Checking condition {}", metadata.name());
        if !metadata.is_parameter_set(&config.profile.parameters) {
            println!(
                "Please set profile condition parameter: {}",
                metadata.name()
            );
            metadata.interactive_set_parameter(&mut config.profile.parameters)?;
            parameter_modified = true;
        }
    }
    if parameter_modified {
        log::info!("Saving modified profile parameters.");
        config.dump()?;
    }
    Ok(())
}
