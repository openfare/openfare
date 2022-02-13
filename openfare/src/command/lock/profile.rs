use super::common;
use crate::common::config::FileStore;
use anyhow::{format_err, Result};
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

    let payee = crate::profile::Profile::load()?;
    if let Some((name, _payee)) =
        openfare_lib::lock::payee::get_lock_payee(&*payee, &lock_handle.lock.payees)
    {
        add_shares_to_plans(&name, args.shares, &plan_ids, &mut lock_handle);
    } else {
        // Add payee to lock.
        let name = if lock_handle.lock.payees.contains_key(&(*payee).label) {
            openfare_lib::lock::payee::unique_name(&(*payee).label, &payee)
        } else {
            (*payee).label.clone()
        };
        lock_handle
            .lock
            .payees
            .insert(name.clone(), (*payee).clone());

        add_shares_to_plans(&name, args.shares, &plan_ids, &mut lock_handle);
    }
    Ok(())
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
    /// Payment plan ID(s). All plans included if unset.
    #[structopt(long, short)]
    pub id: Vec<usize>,

    /// Payee name label(s). Removes local payee only if unset.
    #[structopt(name = "name", long = "name", short)]
    pub names: Vec<String>,

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
