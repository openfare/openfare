use crate::common::fs::FileStore;
use anyhow::{format_err, Result};
use std::convert::TryFrom;

use crate::config::Config;
use crate::extensions::{common, process};
mod github;

pub fn add_from_url(
    url: &url::Url,
    extensions_bin_directory: &std::path::PathBuf,
) -> Result<String> {
    let archive_url = if is_supported_archive_url(&url)? {
        url.clone()
    } else {
        match get_archive_url(&url)? {
            Some(url) => url,
            None => {
                return Err(format_err!(
                    "Failed to obtain suitable release archive URL."
                ))
            }
        }
    };
    log::info!("Using archive URL: {}", archive_url);

    let archive_type = crate::common::fs::archive::ArchiveType::try_from(
        &std::path::PathBuf::from(archive_url.path()),
    )?;

    let tmp_dir = tempdir::TempDir::new("openfare_extension_add")?;
    let tmp_directory_path = tmp_dir.path().to_path_buf();
    log::info!(
        "Downloading extension archive to temporary directory: {}",
        tmp_directory_path.display()
    );
    let archive_path =
        tmp_directory_path.join(format!("archive.{}", archive_type.try_to_string()?));

    crate::common::fs::archive::download(&archive_url, &archive_path)?;
    crate::common::fs::archive::extract(&archive_path, &tmp_directory_path)?;

    let (bin_path, extension_name) = get_bin_file_metadata(&tmp_directory_path)?.ok_or(
        format_err!("Failed to identify extension binary in archive."),
    )?;
    log::info!(
        "Identified binary for extension {}: {}",
        extension_name,
        bin_path.display()
    );
    let bin_file_name = bin_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or(format_err!("Failed to derive extension binary file name."))?;

    let bin_destination_path = extensions_bin_directory.join(bin_file_name);
    log::info!("Copying binary to path: {}", bin_destination_path.display());
    std::fs::copy(&bin_path, &bin_destination_path)?;

    ensure_executable_permissions(&bin_destination_path)?;

    tmp_dir.close()?;
    Ok(extension_name)
}

#[cfg(target_family = "unix")]
fn ensure_executable_permissions(path: &std::path::PathBuf) -> Result<()> {
    log::debug!(
        "Setting executable permissions to 755 for file: {}",
        path.display()
    );
    use std::os::unix::fs::PermissionsExt;
    let permissions = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(&path, permissions)?;
    Ok(())
}

#[cfg(not(target_family = "unix"))]
fn ensure_executable_permissions(_path: &std::path::PathBuf) -> Result<()> {
    Ok(())
}

fn get_bin_file_metadata(
    directory: &std::path::PathBuf,
) -> Result<Option<(std::path::PathBuf, String)>> {
    let regex_pattern = get_bin_name_regex()?;
    for entry in std::fs::read_dir(&directory)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = get_name_from_bin(&path, &regex_pattern)? {
            return Ok(Some((path, name)));
        }
    }
    Ok(None)
}

fn get_bin_name_regex() -> Result<regex::Regex> {
    Ok(regex::Regex::new(
        r"openfare-(?P<name>[a-zA-Z0-9]*)(\.exe)?$",
    )?)
}

fn get_name_from_bin(
    path: &std::path::PathBuf,
    regex_pattern: &regex::Regex,
) -> Result<Option<String>> {
    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
        match regex_pattern.captures(file_name) {
            Some(captures) => Ok(Some(captures["name"].to_string())),
            None => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn is_supported_archive_url(url: &url::Url) -> Result<bool> {
    let path = std::path::PathBuf::from(url.path());
    Ok(crate::common::fs::archive::ArchiveType::try_from(&path)?
        != crate::common::fs::archive::ArchiveType::Unknown)
}

/// Returns a release archive URL.
fn get_archive_url(url: &url::Url) -> Result<Option<url::Url>> {
    Ok(if url.host_str() == Some("github.com") {
        github::get_archive_url(&url)?
    } else {
        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_extension_bin_name {
        use super::*;

        #[test]
        fn test_matching_file_name() -> Result<()> {
            let regex_pattern = get_bin_name_regex()?;
            let bin_path =
                std::path::PathBuf::from("/tmp/openfare-extension_add/openfare-python/openfare-py");
            let result = get_name_from_bin(&bin_path, &regex_pattern)?;
            let expected = Some("py".to_string());
            assert_eq!(result, expected);
            Ok(())
        }

        #[test]
        fn test_not_matching_file_name() -> Result<()> {
            let regex_pattern = get_bin_name_regex()?;
            let bin_path = std::path::PathBuf::from(
                "/tmp/openfare-extension_add/openfare-python/openfare-py.d",
            );
            let result = get_name_from_bin(&bin_path, &regex_pattern)?;
            let expected = None;
            assert_eq!(result, expected);
            Ok(())
        }
    }
}

/// Update config with discoverable extensions.
pub fn update_config(config: &mut Config) -> Result<()> {
    log::debug!("Discover extensions and update config.");

    let extensions = process::get_all()?;
    let extension_name_map: std::collections::BTreeMap<_, _> = extensions
        .iter()
        .map(|extension| (extension.name(), extension))
        .collect();

    let all_found_names: std::collections::BTreeSet<_> =
        extension_name_map.keys().cloned().collect();

    let configured_names: std::collections::BTreeSet<_> = config
        .extensions
        .enabled
        .keys()
        .map(|name| name.clone())
        .collect();

    let stale_names: Vec<_> = configured_names.difference(&all_found_names).collect();
    let registries_map = config.extensions.registries.clone();
    for name in &stale_names {
        config.extensions.enabled.remove(name.clone());

        // Update registries map.
        for (registry, extension_name) in &registries_map {
            if *extension_name == **name {
                config.extensions.registries.remove(registry);
            }
        }
    }

    let new_names: Vec<_> = all_found_names.difference(&configured_names).collect();
    for name in &new_names {
        config.extensions.enabled.insert((**name).clone(), true);

        // Update registries map.
        if let Some(extension) = extension_name_map.get(name.as_str()) {
            for registry in extension.registries() {
                config
                    .extensions
                    .registries
                    .insert(registry, (*name).clone());
            }
        }
    }

    if !stale_names.is_empty() || !new_names.is_empty() {
        config.dump()?;
    }
    Ok(())
}

/// Enable extension.
pub fn enable(name: &str, config: &mut Config) -> Result<()> {
    if let Some(enabled_status) = config.extensions.enabled.get_mut(&name.to_string()) {
        *enabled_status = true;
        config.dump()?;
        Ok(())
    } else {
        Err(format_err!("Failed to find extension."))
    }
}

/// Disable extension.
pub fn disable(name: &str, config: &mut Config) -> Result<()> {
    if let Some(enabled_status) = config.extensions.enabled.get_mut(&name.to_string()) {
        *enabled_status = false;
        config.dump()?;
        Ok(())
    } else {
        Err(format_err!("Failed to find extension."))
    }
}

pub fn remove(name: &str) -> Result<()> {
    let mut config = Config::load()?;
    update_config(&mut config)?;

    let all_extension_names = get_all_names(&config)?;
    if !all_extension_names.contains(name) {
        return Err(format_err!(
            "Failed to find extension. Known extensions: {}",
            all_extension_names
                .into_iter()
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    // Remove extension specific config file.
    let path = common::get_config_path(&name)?;
    if path.is_file() {
        log::info!("Removing extension config file: {}", path.display());
        std::fs::remove_file(&path)?;
    }

    // Remove extension process file.
    let extension_paths = process::get_extension_paths()?;
    if let Some(path) = extension_paths.get(name) {
        log::info!("Deleting extension bin file: {}", path.display());
        std::fs::remove_file(&path)?;
    }

    update_config(&mut config)?;
    Ok(())
}

/// Given an extension's name, returns true if the extension is enabled. Otherwise returns false.
pub fn is_enabled(name: &str, config: &Config) -> Result<bool> {
    Ok(*config.extensions.enabled.get(name).unwrap_or(&false))
}

/// Returns enabled extensions.
pub fn get_enabled(
    names: &std::collections::BTreeSet<String>,
    config: &Config,
) -> Result<Vec<Box<dyn openfare_lib::extension::Extension>>> {
    log::debug!("Identifying enabled extensions.");
    let extensions = process::get_all()?
        .into_iter()
        .filter(|extension| {
            *config
                .extensions
                .enabled
                .get(&extension.name())
                .unwrap_or(&false)
        })
        .filter(|extension| names.contains(&extension.name()))
        .collect();

    Ok(extensions)
}

/// Returns a set of all enabled installed extensions by names.
pub fn get_enabled_names(config: &Config) -> Result<std::collections::BTreeSet<String>> {
    Ok(config
        .extensions
        .enabled
        .iter()
        .filter(|(_name, enabled_flag)| **enabled_flag)
        .map(|(name, _enabled_flag)| name.clone())
        .collect())
}

pub fn get_all_names(config: &Config) -> Result<std::collections::BTreeSet<String>> {
    Ok(config
        .extensions
        .enabled
        .iter()
        .map(|(name, _enabled_flag)| name.clone())
        .collect())
}

/// Check given extensions are enabled. If not specified select all enabled extensions.
pub fn handle_extension_names_arg(
    extension_names: &Option<Vec<String>>,
    config: &Config,
) -> Result<std::collections::BTreeSet<String>> {
    let names = match &extension_names {
        Some(extension_names) => {
            let disabled_names: Vec<_> = extension_names
                .iter()
                .cloned()
                .filter(|name| !is_enabled(&name, &config).unwrap_or(false))
                .collect();
            if !disabled_names.is_empty() {
                return Err(format_err!(
                    "The following disabled extensions were given: {}",
                    disabled_names.join(", ")
                ));
            } else {
                extension_names.into_iter().cloned().collect()
            }
        }
        None => get_enabled_names(&config)?,
    };
    log::debug!("Using extensions: {:?}", names);
    Ok(names)
}

/// Clean extension name.
///
/// Example: openfare-py --> py
pub fn clean_name(name: &str) -> String {
    match &name.strip_prefix(process::EXTENSION_FILE_NAME_PREFIX) {
        Some(name) => name.to_string(),
        None => name.to_string(),
    }
}
