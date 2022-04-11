use super::common;
use crate::common::fs::FileStore;
use anyhow::{format_err, Result};
use std::str::FromStr;
use structopt::{self, StructOpt};

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct AddArguments {
    /// Number of payment split shares.
    #[structopt(long, short)]
    pub shares: u64,

    /// Payee profile URL.
    #[structopt(long, short)]
    pub url: Option<String>,

    /// Payee label.
    #[structopt(long, short)]
    pub label: Option<String>,

    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

pub fn add(args: &AddArguments) -> Result<()> {
    let mut lock_handle = crate::handles::LockHandle::load(&args.lock_file_args.path)?;
    if lock_handle.lock.plans.is_empty() {
        return Err(format_err!(
            "No payment plan found. Add a plan first: openfare lock add plan"
        ));
    }

    let profile = get_profile(&args.url)?;

    // Profile already included in lock as payee. Add shares only.
    if let Some((label, _payee)) =
        openfare_lib::lock::payee::get_lock_payee(&(*profile).clone(), &lock_handle.lock.payees)
    {
        add_shares(&label, args.shares, &mut lock_handle);
        return Ok(());
    }

    let payee = get_payee(&profile);

    // Derive unique label.
    let label = get_label(&args.label, &profile)?;
    let label = if lock_handle.lock.payees.contains_key(&label) {
        openfare_lib::lock::payee::unique_label(&label, &payee)
    } else {
        label.clone()
    };

    lock_handle.lock.payees.insert(label.clone(), payee);
    add_shares(&label, args.shares, &mut lock_handle);
    Ok(())
}

fn get_payee(profile: &crate::handles::ProfileHandle) -> openfare_lib::lock::payee::Payee {
    let url = if let Some(from_url_status) = &profile.from_url_status {
        match from_url_status.method {
            crate::handles::profile::FromUrlMethod::Git => {
                // Prefer HTTPS git URL in lock.
                from_url_status
                    .url
                    .git
                    .as_https_url()
                    .or(Some(from_url_status.url.original.clone()))
            }
            crate::handles::profile::FromUrlMethod::HttpGetJson => {
                Some(from_url_status.url.to_string())
            }
        }
    } else {
        None
    };

    openfare_lib::lock::payee::Payee {
        url,
        profile: (**profile).clone(),
    }
}

fn get_profile(url: &Option<String>) -> Result<crate::handles::ProfileHandle> {
    // Parse URL argument.
    let url = if let Some(url) = &url {
        Some(crate::common::url::Url::from_str(&url)?)
    } else {
        None
    };
    Ok(if let Some(url) = &url {
        crate::handles::ProfileHandle::from_url(&url)?
    } else {
        crate::handles::ProfileHandle::load()?
    })
}

/// Get payee label from label argument or URL.
fn get_label(
    label_arg: &Option<String>,
    profile: &crate::handles::ProfileHandle,
) -> Result<String> {
    let url_label = if let Some(from_url_status) = &profile.from_url_status {
        match from_url_status.method {
            crate::handles::profile::FromUrlMethod::Git => from_url_status.url.git.username.clone(),
            crate::handles::profile::FromUrlMethod::HttpGetJson => None,
        }
    } else {
        None
    };
    let label = label_arg.clone().or(url_label);
    if let Some(label) = label {
        Ok(label)
    } else {
        Err(anyhow::format_err!(
            "Failed to derive payee label from known URLs, please specify using --label."
        ))
    }
}

fn add_shares(payee_label: &str, payee_shares: u64, lock_handle: &mut crate::handles::LockHandle) {
    if let Some(shares) = &mut lock_handle.lock.shares {
        shares.insert(payee_label.to_string(), payee_shares);
    } else {
        lock_handle.lock.shares = Some(maplit::btreemap! {payee_label.to_string() => payee_shares})
    }
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct RemoveArguments {
    /// Payee profile label(s). If unset, removes payee corresponding to local profile only.
    #[structopt(name = "label")]
    pub labels: Vec<String>,

    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

pub fn remove(args: &RemoveArguments) -> Result<()> {
    let mut lock_handle = crate::handles::LockHandle::load(&args.lock_file_args.path)?;

    // If no payee labels given, use local payee label.
    let labels = if args.labels.is_empty() {
        get_lock_local_payee(&lock_handle)?
            .and_then(|label| Some(vec![label]))
            .unwrap_or_default()
    } else {
        args.labels.clone()
    };

    for label in labels {
        // Remove from payees.
        lock_handle.lock.payees.remove(&label);
        // Remove from shares.
        if let Some(shares) = &mut lock_handle.lock.shares {
            shares.remove(&label);
        }
    }

    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct UpdateArguments {
    /// Payee profile label(s). If unset, updates all payee profiles.
    #[structopt(name = "label")]
    pub labels: Vec<String>,

    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

pub fn update(args: &UpdateArguments) -> Result<()> {
    let mut lock_handle = crate::handles::LockHandle::load(&args.lock_file_args.path)?;
    let local_payee_label = get_lock_local_payee(&lock_handle)?;

    for (label, payee) in &mut lock_handle.lock.payees {
        // Skip label if labels given as argument and label wasn't given.
        if !args.labels.is_empty() && !args.labels.contains(label) {
            continue;
        }

        // Update local profile with local data rather than via URL.
        if let Some(local_payee_label) = &local_payee_label {
            if label == local_payee_label {
                let profile = crate::handles::ProfileHandle::load()?;
                let latest_profile = (*profile).clone();
                if payee.profile != latest_profile {
                    log::debug!("Updating local profile: {}", label);
                    payee.profile = latest_profile;
                }
                continue;
            }
        }

        if let Some(url) = &payee.url {
            let url = crate::common::url::Url::from_str(&url)?;
            let latest_profile = (*crate::handles::ProfileHandle::from_url(&url)?).clone();
            if payee.profile != latest_profile {
                log::debug!("Updating profile: {}", label);
                payee.profile = latest_profile;
            }
        }
    }
    Ok(())
}

/// Returns the payee label associated with the local profile.
fn get_lock_local_payee(
    lock_handle: &crate::handles::LockHandle,
) -> Result<Option<openfare_lib::lock::payee::Label>> {
    let profile = crate::handles::ProfileHandle::load()?;
    let label = if let Some((label, _)) =
        openfare_lib::lock::payee::get_lock_payee(&*profile, &lock_handle.lock.payees)
    {
        Some(label.clone())
    } else {
        None
    };
    Ok(label)
}
