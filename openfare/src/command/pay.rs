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
    /// Donation. Example: 1usd or 200sats
    pub donation: Option<openfare_lib::price::Price>,

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

    let extensions = extensions::manage::from_names_arg(&args.extension_names, &config)?;
    let all_extension_locks = get_locks(&extensions, &extension_args)?;
    let mut items = vec![];
    for extension_locks in all_extension_locks {
        if !openfare_lib::lock::plan::conditions::parameters::check_set(
            &extension_locks.package_locks.conditions_metadata(),
            &mut config.profile.parameters,
        )? {
            config.dump()?;
        }

        let basket_items = get_basket_items(&extension_locks, &config)?;
        items.extend(basket_items);
    }
    crate::services::pay(&args.donation, &items, &args.service, &config)?;
    Ok(())
}

pub struct ExtensionLocks {
    pub extension_name: String,
    pub package_locks: openfare_lib::package::PackageLocks,
}

/// Get dependencies locks from all extensions.
pub fn get_locks(
    extensions: &Vec<Box<dyn openfare_lib::extension::Extension>>,
    extension_args: &Vec<String>,
) -> Result<Vec<ExtensionLocks>> {
    let working_directory = std::env::current_dir()?;
    log::debug!("Current working directory: {}", working_directory.display());
    let extensions_results = crate::extensions::project::dependencies_locks(
        &working_directory,
        &extensions,
        &extension_args,
    )?;
    let extensions_results =
        crate::extensions::common::filter_results(&extensions, &extensions_results)?;

    let all_extension_locks: Vec<_> = extensions_results
        .iter()
        .map(|(extension, extension_result)| {
            let package_locks = extension_result
                .package_locks
                .filter_valid_dependencies_locks();
            ExtensionLocks {
                extension_name: extension.name(),
                package_locks,
            }
        })
        .collect();
    Ok(all_extension_locks)
}

/// Get applicable payment plans from dependencies packages.
pub fn get_basket_items(
    extension_locks: &ExtensionLocks,
    config: &crate::config::Config,
) -> Result<Vec<openfare_lib::api::services::basket::Item>> {
    let mut basket_items: Vec<_> = vec![];
    for (package, lock) in &extension_locks.package_locks.dependencies_locks {
        let lock = match lock {
            Some(lock) => lock,
            None => continue,
        };
        let plans =
            openfare_lib::lock::plan::filter_applicable(&lock.plans, &config.profile.parameters)?;
        if plans.is_empty() {
            // Skip package if no applicable plans found.
            continue;
        }

        let total_price = plans
            .iter()
            .map(|(_id, plan)| plan.price.clone().unwrap_or_default())
            .sum();

        let item = openfare_lib::api::services::basket::Item {
            package: package.clone(),
            extension_name: extension_locks.extension_name.to_string(),
            plans,
            total_price,
            payees: lock.payees.clone(),
            shares: lock.shares.clone(),
        };
        basket_items.push(item);
    }
    Ok(basket_items)
}
