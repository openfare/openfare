use anyhow::Result;

/// Get package dependencies locks.
///
/// Conducts a parallel search across extensions.
pub fn dependencies_locks(
    package_name: &str,
    package_version: &Option<&str>,
    extensions: &Vec<Box<dyn openfare_lib::extension::Extension>>,
    extension_args: &Vec<String>,
) -> Result<
    Vec<
        Result<
            openfare_lib::extension::commands::package_dependencies_locks::PackageDependenciesLocks,
        >,
    >,
> {
    crossbeam_utils::thread::scope(|s| {
        let mut threads = Vec::new();
        for extension in extensions {
            threads.push(s.spawn(move |_| {
                extension.package_dependencies_locks(
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
