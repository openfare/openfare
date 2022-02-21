use crate::common::fs::FileStore;
use anyhow::Result;
use structopt::{self, StructOpt};

use crate::extensions;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct Arguments {
    /// Specify payment service.
    #[structopt(long, short)]
    pub service: Option<crate::services::Service>,

    /// Set specified payment service as default.
    #[structopt(long, short)]
    pub default: bool,

    /// Specify an extension for handling the package and its dependencies.
    /// Example values: py, js, rs
    #[structopt(long = "extension", short = "e", name = "name")]
    pub extension_names: Option<Vec<String>>,
}

pub fn run_command(args: &Arguments, extension_args: &Vec<String>) -> Result<()> {
    let mut config = crate::config::Config::load()?;
    extensions::manage::update_config(&mut config)?;
    if args.default {
        if let Some(service) = &args.service {
            config.services.default = service.clone();
            config.dump()?;
        }
    }

    let config = config;
    let extension_names =
        extensions::manage::handle_extension_names_arg(&args.extension_names, &config)?;

    let all_extension_locks = get_dependencies_locks(&extension_names, &extension_args, &config)?;

    crate::services::pay(&all_extension_locks, &args.service, &config)?;
    Ok(())
}

/// Get dependencies locks from all extensions.
fn get_dependencies_locks(
    extension_names: &std::collections::BTreeSet<String>,
    extension_args: &Vec<String>,
    config: &crate::config::Config,
) -> Result<Vec<crate::services::common::ExtensionLocks>> {
    let working_directory = std::env::current_dir()?;
    log::debug!("Current working directory: {}", working_directory.display());

    let extensions = extensions::manage::get_enabled(&extension_names, &config)?;
    let extensions_results = extensions::fs_defined_dependencies_locks(
        &working_directory,
        &extensions,
        &extension_args,
    )?;

    let extension_dependencies_locks: Vec<_> = extensions
        .iter()
        .zip(extensions_results.iter())
        .filter_map(|(extension, extension_result)| {
            log::debug!(
                "Inspecting package OpenFare locks found by extension: {name} ({version})",
                name = extension.name(),
                version = extension.version()
            );

            let locks = match extension_result {
                Ok(locks) => locks,
                Err(error) => {
                    log::error!(
                        "Extension {name} error: {error}",
                        name = extension.name(),
                        error = error
                    );
                    return None;
                }
            };
            let dependencies_locks: std::collections::BTreeMap<_, _> = locks
                .package_locks
                .dependencies_locks
                .iter()
                .filter_map(|(name, lock)| {
                    if let Some(lock) = lock {
                        Some((name.clone(), lock.clone()))
                    } else {
                        None
                    }
                })
                .collect();

            Some(crate::services::common::ExtensionLocks {
                extension_name: extension.name(),
                package_locks: dependencies_locks,
            })
        })
        .collect();
    Ok(extension_dependencies_locks)
}
