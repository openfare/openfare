use crate::extensions;
use anyhow::Result;

pub struct ExtensionLocks {
    pub extension_name: String,
    pub package_locks:
        std::collections::BTreeMap<openfare_lib::package::Package, openfare_lib::lock::Lock>,
}

/// Get dependencies locks from all extensions.
pub fn get_locks(
    extensions: &Vec<Box<dyn openfare_lib::extension::Extension>>,
    extension_args: &Vec<String>,
    _config: &crate::config::Config,
) -> Result<Vec<ExtensionLocks>> {
    let working_directory = std::env::current_dir()?;
    log::debug!("Current working directory: {}", working_directory.display());
    let extensions_results =
        extensions::project::dependencies_locks(&working_directory, &extensions, &extension_args)?;

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

            Some(ExtensionLocks {
                extension_name: extension.name(),
                package_locks: dependencies_locks,
            })
        })
        .collect();
    Ok(extension_dependencies_locks)
}

/// Get applicable payment plans from packages.
pub fn get_packages_plans(
    extension_name: &str,
    package_locks: &std::collections::BTreeMap<
        openfare_lib::package::Package,
        openfare_lib::lock::Lock,
    >,
    config: &crate::config::Config,
) -> Result<Vec<openfare_lib::api::services::basket::Item>> {
    let mut packages_plans: Vec<_> = vec![];
    for (package, lock) in package_locks {
        let plans =
            openfare_lib::lock::plan::filter_applicable(&lock.plans, &config.profile.parameters)?;
        if plans.is_empty() {
            // Skip package if no applicable plans found.
            continue;
        }

        let total_price = plans
            .iter()
            .map(|(_id, plan)| plan.payments.total.clone().unwrap_or_default())
            .sum();

        let order_item = openfare_lib::api::services::basket::Item {
            package: package.clone(),
            extension_name: extension_name.to_string(),
            plans,
            total_price,
            payees: lock.payees.clone(),
        };
        packages_plans.push(order_item);
    }
    Ok(packages_plans)
}
