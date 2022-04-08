use anyhow::Result;

use super::{common, format};
use crate::extensions;

/// Returns price information for a project and its dependencies.
pub fn price(
    extensions: &Vec<Box<dyn openfare_lib::extension::Extension>>,
    extension_args: &Vec<String>,
    config: &crate::config::Config,
) -> Result<()> {
    let working_directory = std::env::current_dir()?;
    log::debug!("Current working directory: {}", working_directory.display());
    let extensions_results =
        extensions::project::dependencies_locks(&working_directory, &extensions, &extension_args)?;

    let mut locks_found = false;

    for (_extension, extension_result) in
        extensions::common::filter_results(&extensions, &extensions_results)?
    {
        locks_found |= extension_result.package_locks.has_locks();
        if let Some(price_report) = common::get_report(&extension_result.package_locks, &config)? {
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

pub fn query_extensions<'a>(
    extensions: &'a Vec<Box<dyn openfare_lib::extension::Extension>>,
    extension_args: &Vec<String>,
) -> Result<
    Vec<(
        &'a Box<dyn openfare_lib::extension::Extension>,
        openfare_lib::extension::commands::project_dependencies_locks::ProjectDependenciesLocks,
    )>,
> {
    let working_directory = std::env::current_dir()?;
    log::debug!("Current working directory: {}", working_directory.display());
    let extensions_results =
        extensions::project::dependencies_locks(&working_directory, &extensions, &extension_args)?;
    Ok(
        extensions::common::filter_results(&extensions, &extensions_results)?
            .into_iter()
            .map(|(extension, result)| (extension, result.to_owned()))
            .collect(),
    )
}
