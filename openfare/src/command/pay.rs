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

    let all_extension_dependency_locks =
        get_dependencies_locks(&extension_names, &extension_args, &config)?;

    let mut order_items = std::collections::BTreeMap::<_, _>::new();
    for extension_dependency_locks in all_extension_dependency_locks {
        let packages_plans =
            get_packages_plans(&extension_dependency_locks.package_locks, &config)?;
        order_items.insert(extension_dependency_locks.extension_name, packages_plans);
    }

    let order = openfare_lib::api::portal::checkout::Order {
        items: order_items,
        api_key: config.portal.api_key.clone(),
    };

    if order.is_empty() {
        println!("No applicable payment plans found.");
        return Ok(());
    }

    let checkout_url = submit_order(&order, &config)?;
    println!("Checkout via URL:\n{}", checkout_url);
    Ok(())
}

struct ExtensionDependenciesLocks {
    pub extension_name: String,
    pub package_locks:
        std::collections::BTreeMap<openfare_lib::package::Package, openfare_lib::lock::Lock>,
}

/// Get dependencies locks from all extensions.
fn get_dependencies_locks(
    extension_names: &std::collections::BTreeSet<String>,
    extension_args: &Vec<String>,
    config: &common::config::Config,
) -> Result<Vec<ExtensionDependenciesLocks>> {
    let working_directory = std::env::current_dir()?;
    log::debug!("Current working directory: {}", working_directory.display());

    let extensions = extension::manage::get_enabled(&extension_names, &config)?;
    let extensions_results =
        extension::fs_defined_dependencies_locks(&working_directory, &extensions, &extension_args)?;

    let extension_dependencies_locks: Vec<_> = extensions
        .iter()
        .zip(extensions_results.iter())
        .filter_map(|(extension, extension_result)| {
            log::debug!(
                "Inspecting package OpenFare locks found by extension: {}",
                extension.name()
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

            Some(ExtensionDependenciesLocks {
                extension_name: extension.name(),
                package_locks: dependencies_locks,
            })
        })
        .collect();
    Ok(extension_dependencies_locks)
}

/// Get applicable payment plans from packages.
fn get_packages_plans(
    package_locks: &std::collections::BTreeMap<
        openfare_lib::package::Package,
        openfare_lib::lock::Lock,
    >,
    config: &common::config::Config,
) -> Result<Vec<openfare_lib::api::portal::checkout::PackagePlans>> {
    let mut packages_plans: Vec<_> = vec![];
    for (package, lock) in package_locks {
        let plans = openfare_lib::lock::plan::filter_applicable(&lock.plans, &config.profile)?;
        if plans.is_empty() {
            // Skip package if no applicable plans found.
            continue;
        }

        let plans: Vec<_> = plans
            .into_iter()
            .map(|(plan_id, plan)| openfare_lib::api::portal::checkout::Plan { plan_id, plan })
            .collect();

        let order_item = openfare_lib::api::portal::checkout::PackagePlans {
            package: package.clone(),
            plans,
            payees: lock.payees.clone(),
        };
        packages_plans.push(order_item);
    }
    Ok(packages_plans)
}

fn submit_order(
    order: &openfare_lib::api::portal::checkout::Order,
    config: &common::config::Config,
) -> Result<url::Url> {
    let client = reqwest::blocking::Client::new();
    let url = config
        .portal
        .url
        .join(&openfare_lib::api::portal::checkout::ROUTE)?;

    log::debug!("Submitting orders: {:?}", order);
    log::debug!("HTTP POST orders to endpoint: {}", url);
    let response = client.post(url.clone()).json(&order).send()?;
    if response.status() != 200 {
        return Err(anyhow::format_err!(
            "Portal response error ({status}):\n{url}",
            status = response.status(),
            url = url.to_string()
        ));
    }

    let response: openfare_lib::api::portal::checkout::Response = response.json()?;
    Ok(response.checkout_url)
}
