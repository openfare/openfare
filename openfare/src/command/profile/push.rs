use crate::common::fs::FileStore;
use anyhow::Result;
use std::str::FromStr;
use structopt::{self, StructOpt};

#[derive(Debug, StructOpt, Clone)]
pub struct Arguments {
    /// Git repository URL. If unset, uses current profile URL.
    url: Option<String>,
}

pub fn push(args: &Arguments) -> Result<()> {
    let mut config = crate::config::Config::load()?;
    let url = if let Some(url) = args.url.clone() {
        let url = crate::common::url::Url::from_str(&url)?;
        if config.profile.url.is_none() {
            config.profile.url = Some(url.original.clone());
        }
        url
    } else {
        if let Some(url) = config.profile.url.clone() {
            crate::common::url::Url::from_str(&url)?
        } else {
            return Err(anyhow::format_err!("Failed to find URL. Not found in config profile and not given as argument.\nSet profile URL using: openfare config set profile.url <url>"));
        }
    };

    let tmp_dir = tempdir::TempDir::new("openfare_profile_push")?;
    let tmp_directory_path = tmp_dir.path().to_path_buf();
    log::debug!("Using temp directory: {}", tmp_directory_path.display());

    clone_repo(&url, &tmp_directory_path)?;

    let profile = crate::handles::ProfileHandle::load()?;
    let remote_profile = (*profile).clone().into();

    insert_profile(&remote_profile, &tmp_directory_path)?;
    push_repo(&tmp_directory_path)?;
    println!("Profile push complete.");

    // Write updated config profile to disk only if all git operations succeed.
    config.dump()?;
    Ok(())
}

fn clone_repo(
    url: &crate::common::url::Url,
    tmp_directory_path: &std::path::PathBuf,
) -> Result<()> {
    let url = if let Some(url) = url.git.as_ssh_url() {
        url
    } else {
        url.original.clone()
    };
    println!("Cloning repository for writing profile: {}", url.as_str());
    crate::common::git::run_command(
        vec!["clone", "--depth", "1", url.as_str(), "."],
        &tmp_directory_path,
    )?;
    Ok(())
}

fn push_repo(tmp_directory_path: &std::path::PathBuf) -> Result<()> {
    println!("Pushing local clone.");
    crate::common::git::run_command(vec!["add", "-A"], &tmp_directory_path)?;
    crate::common::git::commit("Update OpenFare profile.", &tmp_directory_path)?;
    crate::common::git::run_command(vec!["push", "origin"], &tmp_directory_path)?;
    Ok(())
}

fn insert_profile(
    remote_profile: &openfare_lib::profile::RemoteProfile,
    directory_path: &std::path::PathBuf,
) -> Result<()> {
    let path = directory_path.join(openfare_lib::profile::FILE_NAME);
    println!("Writing profile to local clone: {}", path.display());
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
    serde_json::to_writer_pretty(writer, &remote_profile)?;
    Ok(())
}
