use anyhow::Result;

use crate::common;
use crate::extension;

use super::format;

/// Returns price information for a project and its dependencies.
pub fn price(
    extension_names: &std::collections::BTreeSet<String>,
    extension_args: &Vec<String>,
    config: &common::config::Config,
) -> Result<()> {
    let working_directory = std::env::current_dir()?;
    log::debug!("Current working directory: {}", working_directory.display());

    let extensions = extension::manage::get_enabled(&extension_names, &config)?;
    let extensions_results =
        extension::fs_defined_dependencies_locks(&working_directory, &extensions, &extension_args)?;

    let mut locks_found = false;

    for (extension, extension_result) in extensions.iter().zip(extensions_results.iter()) {
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

// fn report_dependencies(
//     package_dependencies: &openfare_lib::extension::FileDefinedDependencies,
// ) -> Result<()> {
//     log::info!(
//         "Generating report for dependencies specification file: {}",
//         package_dependencies.path.display()
//     );
//     let dependencies = &package_dependencies.dependencies;

//     let dependency_reports: Result<Vec<report::PriceReport>> = dependencies
//         .into_iter()
//         .map(|dependency| -> Result<report::PriceReport> {
//             Ok(report::get_price_report(
//                 &dependency,
//                 &package_dependencies.registry_host_name,
//             )?)
//         })
//         .collect();
//     let dependency_reports = dependency_reports?;

//     log::info!("Number of dependencies found: {}", dependency_reports.len());
//     if dependency_reports.is_empty() {
//         return Ok(());
//     }

//     let table = table::get(&dependency_reports, false)?;
//     println!(
//         "Registry: {name}\n{path}",
//         name = package_dependencies.registry_host_name,
//         path = package_dependencies.path.display(),
//     );
//     table.printstd();
//     Ok(())
// }
