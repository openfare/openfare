use anyhow::Result;

pub fn get_config_path(extension_name: &str) -> Result<std::path::PathBuf> {
    let config_paths = crate::config::Paths::new()?;
    Ok(config_paths.extensions_directory.join(format!(
        "{extension_name}.json",
        extension_name = extension_name
    )))
}

pub fn filter_results<'a, 'b, T>(
    extensions: &'a Vec<Box<dyn openfare_lib::extension::Extension>>,
    results: &'b Vec<Result<T>>,
) -> Result<Vec<(&'a Box<dyn openfare_lib::extension::Extension>, &'b T)>> {
    let mut filtered_results = vec![];
    for (extension, result) in extensions.iter().zip(results.iter()) {
        log::debug!(
            "Inspecting result from extension: {name} ({version})",
            name = extension.name(),
            version = extension.version()
        );

        let result = match result {
            Ok(result) => {
                log::debug!(
                    "Found Ok result from extension: {name}",
                    name = extension.name(),
                );
                result
            }
            Err(error) => {
                log::error!(
                    "Extension {name} error: {error}",
                    name = extension.name(),
                    error = error
                );
                continue;
            }
        };
        filtered_results.push((extension.clone(), result.clone()));
    }
    Ok(filtered_results)
}
