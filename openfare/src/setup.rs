use anyhow::Result;

/// Ensure setup is complete.
pub fn ensure() -> Result<()> {
    if !is_complete()? {
        setup(false)?;
    }
    Ok(())
}

/// Setup config directory.
///
/// If config file exists and force is false, file will not be modified.
fn setup_config(paths: &crate::config::Paths, force: bool) -> Result<()> {
    std::fs::create_dir_all(&paths.root_directory)?;
    std::fs::create_dir_all(&paths.extensions_directory)?;

    if force || !paths.config_file.is_file() {
        log::debug!("Generating config file: {}", paths.config_file.display());
        let mut config = crate::config::Config::default();
        crate::extension::manage::update_config(&mut config)?;
    } else {
        log::debug!(
            "Not overwriting existing config file (--force: {:?}): {}",
            force,
            paths.config_file.display()
        );
    }
    Ok(())
}

pub fn setup(force: bool) -> Result<()> {
    let config_paths = crate::config::Paths::new()?;
    log::debug!("Using config paths: {:#?}", config_paths);
    setup_config(&config_paths, force)?;
    log::debug!("Config setup complete.");
    Ok(())
}

/// Returns true if setup is complete, otherwise returns false.
///
/// Checks for existence of config file.
pub fn is_complete() -> Result<bool> {
    let config_paths = crate::config::Paths::new()?;
    Ok(config_paths.config_file.is_file())
}
