use super::{common, format};
use crate::extensions;
use anyhow::Result;

/// Prints a price report for a specific package and its dependencies.
pub fn price(
    package_name: &str,
    package_version: &Option<&str>,
    extensions: &Vec<Box<dyn openfare_lib::extension::Extension>>,
    extension_args: &Vec<String>,
    config: &crate::config::Config,
) -> Result<()> {
    let extensions_results = extensions::package::dependencies_locks(
        &package_name,
        &package_version,
        &extensions,
        &extension_args,
    )?;

    let mut locks_found = false;

    for (_extension, extension_result) in
        extensions::common::filter_results(&extensions, &extensions_results)?
    {
        locks_found |= extension_result.package_locks.has_locks();
        if let Some(price_report) = common::get_report(&extension_result.package_locks, &config)? {
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
