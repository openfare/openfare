use anyhow::Result;
use openfare_lib::extension::FromProcess;
use std::collections::HashMap;

use crate::extensions::common;

pub static EXTENSION_FILE_NAME_PREFIX: &str = "openfare-";

/// Discovers and loads process extensions.
pub fn get_all() -> Result<Vec<openfare_lib::extension::process::ProcessExtension>> {
    let extension_paths = get_extension_paths()?;

    let mut threads = vec![];
    for (name, path) in extension_paths.iter() {
        let extension_config_path = common::get_config_path(name)?;
        let process_path = path.clone();

        threads.push(std::thread::spawn(move || {
            openfare_lib::extension::process::ProcessExtension::from_process(
                &process_path,
                &extension_config_path,
            )
        }));
    }
    let extensions: Vec<Result<openfare_lib::extension::process::ProcessExtension>> = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .collect();

    let mut extension_map = HashMap::new();
    for ((_name, path), extension) in extension_paths.into_iter().zip(extensions.into_iter()) {
        extension_map.insert((*path).to_path_buf(), extension);
    }

    let mut valid_extensions = Vec::new();
    for (process_path, extension) in extension_map {
        match extension {
            Ok(v) => {
                valid_extensions.push(v);
            }
            Err(e) => {
                eprintln!(
                    "{extension_name}: Failed to load extension.\n{error}",
                    extension_name = process_path.display(),
                    error = e
                );
            }
        };
    }
    Ok(valid_extensions)
}

pub fn get_extension_paths() -> Result<HashMap<String, std::path::PathBuf>> {
    let mut result: HashMap<String, std::path::PathBuf> = HashMap::new();
    for path in get_candidate_extension_paths()? {
        // Skip non-valid paths.
        if !path.is_dir() && !path.is_file() {
            continue;
        }

        if path.is_file() {
            let name = match get_extension_name(&path)? {
                Some(name) => name,
                None => {
                    continue;
                }
            };
            result.insert(name, path);
            continue;
        }

        // Inspect file in directory. Does not investigate child directories.
        for entry in std::fs::read_dir(path)? {
            let path = entry?.path();
            if path.is_file() {
                let name = match get_extension_name(&path)? {
                    Some(name) => name,
                    None => {
                        continue;
                    }
                };
                result.insert(name, path);
            }
        }
    }
    Ok(result)
}

fn get_candidate_extension_paths() -> Result<Vec<std::path::PathBuf>> {
    let env_path_value = std::env::var_os("PATH").ok_or(anyhow::format_err!(
        "Failed to read PATH environment variable."
    ))?;
    let mut paths = std::env::split_paths(&env_path_value).collect::<Vec<_>>();

    if let Some(extensions_home_directory) = crate::common::fs::get_extensions_default_directory() {
        if extensions_home_directory.exists() {
            paths.push(extensions_home_directory);
        }
    }
    Ok(paths)
}

fn get_extension_name(file_path: &std::path::PathBuf) -> Result<Option<String>> {
    let file_name = file_path
        .file_name()
        .ok_or(anyhow::format_err!("Failed to parse path file name."))?
        .to_str()
        .ok_or(anyhow::format_err!(
            "Failed to parse path file name into string."
        ))?
        .to_string();

    let captures = match regex::Regex::new(&format!(
        "{extension_file_name_prefix}([a-z]*).*",
        extension_file_name_prefix = EXTENSION_FILE_NAME_PREFIX
    ))?
    .captures(file_name.as_str())
    {
        Some(v) => v,
        None => {
            return Ok(None);
        }
    };

    let name = match captures.get(1) {
        Some(v) => v,
        None => {
            return Ok(None);
        }
    }
    .as_str();
    Ok(Some(name.to_string()))
}
