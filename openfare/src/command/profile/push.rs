use crate::common::config::FileStore;
use anyhow::Result;
use std::str::FromStr;
use structopt::{self, StructOpt};

#[derive(Debug, StructOpt, Clone)]
pub struct Arguments {
    /// Git repository URL. If unset, uses current profile URL.
    #[structopt(long, short)]
    url: Option<String>,
}

pub fn push(args: &Arguments) -> Result<()> {
    let mut config = crate::common::config::Config::load()?;
    let url = if let Some(url) = args.url.clone() {
        let url = crate::common::git::GitUrl::from_str(&url)?;
        if config.profile.url.is_none() {
            config.profile.url = Some(url.original_url.clone());
        }
        url
    } else {
        if let Some(url) = config.profile.url.clone() {
            crate::common::git::GitUrl::from_str(&url)?
        } else {
            return Err(anyhow::format_err!("Failed to find URL. Not found in config profile and not given as argument.\nSet profile URL using: openfare config profile.url <url>"));
        }
    };

    let profile = crate::profile::Profile::load()?;

    let tmp_dir = tempdir::TempDir::new("openfare_profile_push")?;
    let tmp_directory_path = tmp_dir.path().to_path_buf();
    log::debug!("Using temp directory: {}", tmp_directory_path.display());

    clone_repo(&url, &tmp_directory_path)?;

    insert_profile(&profile, &tmp_directory_path)?;
    push_repo(&tmp_directory_path)?;

    // Write updated config profile to disk only if all git operations succeed.
    config.dump()?;
    Ok(())
}

fn clone_repo(
    url: &crate::common::git::GitUrl,
    tmp_directory_path: &std::path::PathBuf,
) -> Result<()> {
    let url = if let Some(url) = url.as_ssh_url() {
        url
    } else {
        url.original_url.clone()
    };
    let output = crate::common::git::run_command(
        vec!["clone", "--depth", "1", url.as_str(), "."],
        &tmp_directory_path,
    )?;
    log::debug!("Clone output: {:?}", output);
    Ok(())
}

fn push_repo(tmp_directory_path: &std::path::PathBuf) -> Result<()> {
    let output = crate::common::git::run_command(vec!["add", "-A"], &tmp_directory_path)?;
    log::debug!("Add output: {:?}", output);

    let output = crate::common::git::run_command(
        vec!["commit", "-am", "Update OpenFare profile."],
        &tmp_directory_path,
    )?;
    log::debug!("Commit output: {:?}", output);

    let output = crate::common::git::run_command(vec!["push", "origin"], &tmp_directory_path)?;
    log::debug!("Push output: {:?}", output);
    Ok(())
}

fn insert_profile(
    profile: &crate::profile::Profile,
    directory_path: &std::path::PathBuf,
) -> Result<()> {
    let path = directory_path.join(openfare_lib::profile::FILE_NAME);
    if path.is_file() {
        std::fs::remove_file(&path)?;
    }

    let file = std::fs::OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(&path)
        .expect(format!("Can't open/create file for writing: {}", path.display()).as_str());

    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &*profile)?;
    Ok(())
}
