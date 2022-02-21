use anyhow::{format_err, Context, Result};
use std::io::Read;

use crate::common;

pub fn get_archive_url(repo_url: &url::Url) -> Result<Option<url::Url>> {
    let platform = get_platform()?;
    log::debug!("Identified target platform: {}", platform);

    let releases = get_releases(&repo_url)?;
    if releases.is_empty() {
        log::debug!("Failed to find any releases corresponding to repository URL.");
    } else {
        log::debug!("Found {} candidate releases.", releases.len());
    }

    for release in releases {
        if let Some(assets) = release.get("assets").and_then(|assets| assets.as_array()) {
            for asset in assets {
                if let Some(asset_name) = asset.get("name").and_then(|name| name.as_str()) {
                    if asset_name.contains(&platform) {
                        if let Some(url) = asset
                            .get("browser_download_url")
                            .and_then(|url| url.as_str())
                        {
                            return Ok(Some(url::Url::parse(url)?));
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

/// Get releases given a repository URL such as: https://github.com/openfare/openfare-py
fn get_releases(repo_url: &url::Url) -> Result<Vec<serde_json::Value>> {
    let releases_url = url::Url::parse(
        format!(
            "https://api.github.com/repos{path}/releases",
            path = repo_url.path()
        )
        .as_str(),
    )?;
    log::debug!("Using releases URL: {}", releases_url);

    let client = reqwest::blocking::Client::builder()
        .user_agent(common::HTTP_USER_AGENT)
        .build()?;
    let mut result = client.get(&releases_url.to_string()).send()?;
    let mut body = String::new();
    result.read_to_string(&mut body)?;
    let releases: serde_json::Value =
        serde_json::from_str(&body).context(format!("JSON was not well-formatted:\n{}", body))?;
    let releases = releases
        .as_array()
        .ok_or(format_err!("Failed to find releases from GitHub repo."))?;
    Ok(releases.clone())
}

fn get_platform() -> Result<String> {
    Ok(match std::env::consts::OS {
        "linux" => "unknown-linux-musl",
        "macos" => "apple-darwin",
        "windows" => "pc-windows-msvc",
        other => return Err(format_err!("Unsupported OS type: {}", other)),
    }
    .to_string())
}
