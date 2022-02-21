use super::format;
use crate::extensions;
use anyhow::Result;

/// Prints a price report for a specific package and its dependencies.
pub fn price(
    package_name: &str,
    package_version: &Option<&str>,
    extension_names: &std::collections::BTreeSet<String>,
    extension_args: &Vec<String>,
    config: &crate::config::Config,
) -> Result<()> {
    let extensions = extensions::manage::get_enabled(&extension_names, &config)?;
    let extensions_results = extensions::package_dependencies_locks(
        &package_name,
        &package_version,
        &extensions,
        &extension_args,
    )?;

    let mut locks_found = false;

    for (extension, extension_result) in extensions.iter().zip(extensions_results.iter()) {
        log::debug!(
            "Inspecting package OpenFare locks found by extension: {name} ({version})",
            name = extension.name(),
            version = extension.version()
        );

        let extension_result = match extension_result {
            Ok(d) => d,
            Err(error) => {
                log::error!(
                    "Extension {name} error: {error}",
                    name = extension.name(),
                    error = error
                );
                continue;
            }
        };

        locks_found |= extension_result.package_locks.has_locks();
        if let Some(price_report) =
            crate::price::get_report(&extension_result.package_locks, &config)?
        {
            println!("Registry: {}", extension_result.registry_host_name);
            println!("Total: {}", price_report.price);
            format::print(&price_report, &format::Format::Table, true)?;
            println!("");
        }
    }

    if !locks_found {
        println!("No OpenFare lock file found.")
    }
    Ok(())
}
