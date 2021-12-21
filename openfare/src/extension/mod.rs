use anyhow::Result;
use crossbeam_utils;

mod common;
pub mod manage;
mod process;

/// Identify all supported dependency OpenFare configs which are defined in a local project.
///
/// Conducts a parallel search across extensions.
pub fn fs_defined_dependencies_configs(
    working_directory: &std::path::PathBuf,
    extensions: &Vec<Box<dyn openfare_lib::extension::Extension>>,
    extension_args: &Vec<String>,
) -> Result<Vec<Result<openfare_lib::extension::commands::fs_defined_dependencies_configs::FsDefinedDependenciesConfigs>>>{
    crossbeam_utils::thread::scope(|s| {
        let mut threads = Vec::new();
        for extension in extensions {
            threads.push(s.spawn(move |_| {
                extension.fs_defined_dependencies_configs(&working_directory, &extension_args)
            }));
        }
        let mut result = Vec::new();
        for thread in threads {
            result.push(thread.join().unwrap());
        }
        Ok(result)
    })
    .unwrap()
}

/// Get package dependencies OpenFare configs.
///
/// Conducts a parallel search across extensions.
pub fn package_dependencies_configs(
    package_name: &str,
    package_version: &Option<&str>,
    extensions: &Vec<Box<dyn openfare_lib::extension::Extension>>,
    extension_args: &Vec<String>,
) -> Result<Vec<Result<openfare_lib::extension::commands::package_dependencies_configs::PackageDependenciesConfigs>>>{
    crossbeam_utils::thread::scope(|s| {
        let mut threads = Vec::new();
        for extension in extensions {
            threads.push(s.spawn(move |_| {
                extension.package_dependencies_configs(
                    &package_name,
                    &package_version,
                    &extension_args,
                )
            }));
        }
        let mut result = Vec::new();
        for thread in threads {
            result.push(thread.join().unwrap());
        }
        Ok(result)
    })
    .unwrap()
}
