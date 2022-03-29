use anyhow::Result;

use super::format;
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
