use anyhow::Result;

pub fn get_config_path(extension_name: &str) -> Result<std::path::PathBuf> {
    let config_paths = crate::common::fs::ConfigPaths::new()?;
    Ok(config_paths.extensions_directory.join(format!(
        "{extension_name}.yaml",
        extension_name = extension_name
    )))
}
