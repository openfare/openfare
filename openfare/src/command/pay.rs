use crate::common::config::FileStore;
use anyhow::Result;
use structopt::{self, StructOpt};

use crate::common;
use crate::extension;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct Arguments {
    /// Specify an extension for handling the package and its dependencies.
    /// Example values: py, js, rs
    #[structopt(long = "extension", short = "e", name = "name")]
    pub extension_names: Option<Vec<String>>,
}

pub fn run_command(args: &Arguments, extension_args: &Vec<String>) -> Result<()> {
    let mut config = common::config::Config::load()?;
    extension::manage::update_config(&mut config)?;
    let config = config;
    let extension_names =
        extension::manage::handle_extension_names_arg(&args.extension_names, &config)?;

    Ok(())
}

fn foo(
    extension_names: &std::collections::BTreeSet<String>,
    extension_args: &Vec<String>,
    config: &common::config::Config,
) -> Result<()> {
    let working_directory = std::env::current_dir()?;
    log::debug!("Current working directory: {}", working_directory.display());

    let extensions = extension::manage::get_enabled(&extension_names, &config)?;
    let extensions_results =
        extension::fs_defined_dependencies_locks(&working_directory, &extensions, &extension_args)?;

    let all_extension_results: Vec<_> = extensions
        .iter()
        .zip(extensions_results.iter())
        .filter_map(|(extension, extension_result)| {
            log::debug!(
                "Inspecting package OpenFare locks found by extension: {}",
                extension.name()
            );

            let extension_result = match extension_result {
                Ok(d) => d,
                Err(error) => {
                    log::error!(
                        "Extension {name} error: {error}",
                        name = extension.name(),
                        error = error
                    );
                    return None;
                }
            };

            Some(extension_result)
        })
        .collect();
    Ok(())
}
