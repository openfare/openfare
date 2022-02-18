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
            "No payment plan found. Add a plan first: openfare lock add plan"
        ));
    }

    let plan_ids = args
        .id
        .iter()
        .map(|id| id.to_string())
        .collect::<std::collections::BTreeSet<_>>();

    let profile = get_profile(&args.url)?;

    // Profile already included in lock as payee. Add shares only.
    if let Some((label, _payee)) =
        openfare_lib::lock::payee::get_lock_payee(&(*profile).clone(), &lock_handle.lock.payees)
    {
        add_shares_to_plans(&label, args.shares, &plan_ids, &mut lock_handle);
        return Ok(());
    }

    let payee = get_payee(&profile);

    // Derive unique label.
    let label = get_label(&args.label, &profile)?;
    let label = if lock_handle.lock.payees.contains_key(&label) {
        openfare_lib::lock::payee::unique_name(&label, &payee)
    } else {
        label.clone()
    };

    lock_handle.lock.payees.insert(label.clone(), payee);
    add_shares_to_plans(&label, args.shares, &plan_ids, &mut lock_handle);
    Ok(())
}

fn get_payee(profile: &crate::profile::Profile) -> openfare_lib::lock::payee::Payee {
    let url = if let Some(from_url_status) = &profile.from_url_status {
        match from_url_status.method {
            crate::profile::FromUrlMethod::Git => {
                // Prefer HTTPS git URL in lock.
                from_url_status
                    .url
                    .git
                    .as_https_url()
                    .or(Some(from_url_status.url.original.clone()))
            }
            crate::profile::FromUrlMethod::HttpGetJson => Some(from_url_status.url.to_string()),
        }
    } else {
        None
    };

    openfare_lib::lock::payee::Payee {
        url,
        profile: (**profile).clone(),
    }
}

fn get_profile(url: &Option<String>) -> Result<crate::profile::Profile> {
    // Parse URL argument.
    let url = if let Some(url) = &url {
        Some(crate::common::url::Url::from_str(&url)?)
    } else {
        None
    };
    Ok(if let Some(url) = &url {
        crate::profile::Profile::from_url(&url)?
    } else {
        crate::profile::Profile::load()?
    })
}

/// Get payee label from label argument or URL.
fn get_label(label_arg: &Option<String>, profile: &crate::profile::Profile) -> Result<String> {
    let url_label = if let Some(from_url_status) = &profile.from_url_status {
        match from_url_status.method {
            crate::profile::FromUrlMethod::Git => from_url_status.url.git.username.clone(),
            crate::profile::FromUrlMethod::HttpGetJson => None,
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
