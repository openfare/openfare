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
                if !openfare_lib::lock::plan::conditions::parameters::check_set(
                    &result.package_locks.conditions_metadata(),
                    &mut config.profile.parameters,
                )? {
                    config.dump()?;
                }
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
                if !openfare_lib::lock::plan::conditions::parameters::check_set(
                    &result.package_locks.conditions_metadata(),
                    &mut config.profile.parameters,
                )? {
                    config.dump()?;
                }
            }
            project::price(&extensions, &extension_args, &config)?;
        }
    }
    Ok(())
}
