use super::common;
use anyhow::Result;
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
    pub id: Vec<u8>,

    /// Payment total fee.
    pub total: String,

    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

pub fn add(args: &AddArguments) -> Result<()> {
    let plan_ids = args
        .id
        .iter()
        .map(|id| id.to_string())
        .collect::<std::collections::BTreeSet<_>>();
    let mut lock_handle = common::LockFileHandle::load(&args.lock_file_args.path)?;

    for (_plan_id, plan) in lock_handle
        .lock
        .plans
        .iter_mut()
        .filter(|(id, _plan)| plan_ids.contains(id.as_str()) || plan_ids.is_empty())
    {
        plan.payments.total = Some(openfare_lib::lock::plan::price::Price::try_from(
            args.total.as_str(),
        )?);
    }

    Ok(())
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
    pub id: Vec<u8>,

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

    for (_plan_id, plan) in lock_handle
        .lock
        .plans
        .iter_mut()
        .filter(|(id, _plan)| plan_ids.contains(id.as_str()) || plan_ids.is_empty())
    {
        plan.payments.total = None;
    }
    Ok(())
}
