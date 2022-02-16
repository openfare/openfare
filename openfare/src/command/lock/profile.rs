use super::common;
use crate::common::config::FileStore;
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
    /// Payment plan ID(s). All plans included if unset.
    #[structopt(long, short)]
    pub id: Vec<usize>,

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
    let mut lock_handle = common::LockFileHandle::load(&args.lock_file_args.path)?;
    if lock_handle.lock.plans.is_empty() {
        return Err(format_err!(
            "No payment plan found. Add plan: openfare lock add plan"
        ));
    }

    let plan_ids = args
        .id
        .iter()
        .map(|id| id.to_string())
        .collect::<std::collections::BTreeSet<_>>();

    let url = url(&args.url)?;
    let profile = profile(&url)?;

    if let Some((label, _payee)) =
        openfare_lib::lock::payee::get_lock_payee(&profile, &lock_handle.lock.payees)
    {
        add_shares_to_plans(&label, args.shares, &plan_ids, &mut lock_handle);
    } else {
        let url_str = if let Some(url) = url.clone() {
            // Prefer HTTPS git URL in lock.
            url.as_https_url().or(Some(url.original_url))
        } else {
            None
        };
        // Add payee to lock.
        let payee = openfare_lib::lock::payee::Payee {
            url: url_str,
            profile,
        };

        // Derive unique label.
        let label = label(&args.label, &url)?;
        let label = if lock_handle.lock.payees.contains_key(&label) {
            openfare_lib::lock::payee::unique_name(&label, &payee)
        } else {
            label.clone()
        };

        lock_handle.lock.payees.insert(label.clone(), payee);
        add_shares_to_plans(&label, args.shares, &plan_ids, &mut lock_handle);
    }
    Ok(())
}

/// Get profile from URL or locally.
fn profile(url: &Option<crate::common::git::GitUrl>) -> Result<openfare_lib::profile::Profile> {
    Ok(if let Some(url) = url {
        let tmp_dir = tempdir::TempDir::new("openfare_lock_add_profile")?;
        let tmp_directory_path = tmp_dir.path().to_path_buf();

        let url = if let Some(url) = url.as_ssh_url() {
            url
        } else {
            url.original_url.clone()
        };
        log::debug!("Attempting to clone repository using URL: {}", url);
        let output = crate::common::git::run_command(
            vec!["clone", "--depth", "1", url.as_str(), "."],
            &tmp_directory_path,
        )?;
        log::debug!("Clone output: {:?}", output);
        let path = tmp_directory_path.join(openfare_lib::profile::FILE_NAME);

        if !path.exists() {
            return Err(anyhow::format_err!(
                "Failed to find profile JSON in repository: {}",
                openfare_lib::profile::FILE_NAME
            ));
        }

        let file = std::fs::File::open(&path)?;
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader)?
    } else {
        let profile = crate::profile::Profile::load()?;
        (*profile).clone()
    })
}

/// Returns config profile URL if url not given as argument.
fn url(url_arg: &Option<String>) -> Result<Option<crate::common::git::GitUrl>> {
    let config = crate::common::config::Config::load()?;
    let url = url_arg.clone().or(config.profile.url).and_then(|url| {
        match crate::common::git::GitUrl::from_str(&url) {
            Ok(git_url) => Some(git_url),
            Err(_) => None,
        }
    });
    Ok(url)
}

/// Get payee label from label argument or URL.
fn label(label_arg: &Option<String>, url: &Option<crate::common::git::GitUrl>) -> Result<String> {
    let url_label = url.clone().and_then(|url| url.username);
    let label = label_arg.clone().or(url_label);
    if let Some(label) = label {
        Ok(label)
    } else {
        Err(anyhow::format_err!(
            "Failed to derive payee label from known URLs, please specify using --label."
        ))
    }
}

fn add_shares_to_plans(
    payee_name: &str,
    payee_shares: u64,
    plan_ids: &std::collections::BTreeSet<String>,
    lock_handle: &mut common::LockFileHandle,
) {
    for (_plan_id, plan) in lock_handle
        .lock
        .plans
        .iter_mut()
        .filter(|(id, _plan)| plan_ids.contains(id.as_str()) || plan_ids.is_empty())
    {
        if let Some(shares) = &mut plan.payments.shares {
            shares.insert(payee_name.to_string(), payee_shares);
        } else {
            plan.payments.shares = Some(maplit::btreemap! {payee_name.to_string() => payee_shares})
        }
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
    #[structopt(name = "name")]
    pub names: Vec<String>,

    /// Payment plan ID(s). All plans included if unset.
    #[structopt(long, short)]
    pub id: Vec<usize>,

    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

pub fn remove(args: &RemoveArguments) -> Result<()> {
    let plan_ids = args
        .id
        .iter()
        .map(|id| id.to_string())
        .collect::<std::collections::BTreeSet<_>>();
    let mut lock_handle = common::LockFileHandle::load(&args.lock_file_args.path)?;

    // If no payee names given, use active payee name.
    let names = if args.names.is_empty() {
        get_lock_local_payee(&lock_handle)?
            .and_then(|name| Some(vec![name]))
            .unwrap_or_default()
    } else {
        args.names.clone()
    };

    // Remove from plans.
    for (_plan_id, plan) in lock_handle
        .lock
        .plans
        .iter_mut()
        .filter(|(id, _plan)| plan_ids.contains(id.as_str()) || plan_ids.is_empty())
    {
        for name in &names {
            if let Some(shares) = &mut plan.payments.shares {
                shares.remove(name.as_str());
            }
        }
    }

    // Remove from payees.
    for name in names {
        lock_handle.lock.payees.remove(&name);
    }

    Ok(())
}

fn get_lock_local_payee(
    lock_handle: &common::LockFileHandle,
) -> Result<Option<openfare_lib::lock::payee::Name>> {
    let payee = crate::profile::Profile::load()?;
    let name = if let Some((name, _)) =
        openfare_lib::lock::payee::get_lock_payee(&*payee, &lock_handle.lock.payees)
    {
        Some(name.clone())
    } else {
        None
    };
    Ok(name)
}
