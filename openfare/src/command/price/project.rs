use anyhow::Result;

use super::format;
use crate::extensions;

/// Returns price information for a project and its dependencies.
pub fn price(
    extension_names: &std::collections::BTreeSet<String>,
    extension_args: &Vec<String>,
    config: &crate::config::Config,
) -> Result<()> {
    let working_directory = std::env::current_dir()?;
    log::debug!("Current working directory: {}", working_directory.display());

    let extensions = extensions::manage::get_enabled(&extension_names, &config)?;
    let extensions_results =
        extensions::project::dependencies_locks(&working_directory, &extensions, &extension_args)?;

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
            println!(
                "Project: {path}",
                path = extension_result.project_path.display()
            );
            println!("Total: {}", price_report.price);
            format::print(&price_report, &format::Format::Table, true)?;
            println!("");
        }
    }

    if !locks_found {
        println!("No OpenFare lock files found.")
    }
    Ok(())
}
