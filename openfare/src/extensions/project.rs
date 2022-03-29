use anyhow::Result;

/// Identify all supported dependency locks which are defined in a local project.
///
/// Conducts a parallel search across extensions.
pub fn dependencies_locks(
    working_directory: &std::path::PathBuf,
    extensions: &Vec<Box<dyn openfare_lib::extension::Extension>>,
    extension_args: &Vec<String>,
) -> Result<
    Vec<
        Result<
            openfare_lib::extension::commands::project_dependencies_locks::ProjectDependenciesLocks,
        >,
    >,
> {
    crossbeam_utils::thread::scope(|s| {
        let mut threads = Vec::new();
        for extension in extensions {
            threads.push(s.spawn(move |_| {
                extension.project_dependencies_locks(&working_directory, &extension_args)
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
