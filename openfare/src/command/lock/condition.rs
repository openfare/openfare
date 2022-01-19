use anyhow::Result;
use structopt::{self, StructOpt};

use super::common;

#[derive(Debug, StructOpt, Clone)]
pub struct ConditionArguments {
    /// Current time relative to definite date. Example: "< 2022-01-31"
    #[structopt(name = "current-time", long, short)]
    pub current_time: Option<String>,

    /// Number of employees in the organization. Example: "> 100"
    #[structopt(name = "employees-count", long, short)]
    pub employees_count: Option<String>,
}

impl std::convert::TryInto<openfare_lib::lock::plan::conditions::Conditions>
    for ConditionArguments
{
    type Error = anyhow::Error;
    fn try_into(self) -> Result<openfare_lib::lock::plan::conditions::Conditions, Self::Error> {
        let current_time = if let Some(current_time) = &self.current_time {
            Some(openfare_lib::lock::plan::conditions::CurrentTime::try_from(
                current_time.as_str(),
            )?)
        } else {
            None
        };
        let employees_count = if let Some(employees_count) = &self.employees_count {
            Some(
                openfare_lib::lock::plan::conditions::EmployeesCount::try_from(
                    employees_count.as_str(),
                )?,
            )
        } else {
            None
        };

        Ok(openfare_lib::lock::plan::conditions::Conditions {
            current_time,
            employees_count,
        })
    }
}

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

    #[structopt(flatten)]
    pub conditions: ConditionArguments,

    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

pub fn add(args: &AddArguments) -> Result<()> {
    let conditions = args.conditions.clone().try_into()?;

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
        plan.conditions.set_some(&conditions);
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
    pub id: Vec<usize>,

    /// Number of employees in the organization.
    #[structopt(long = "employees-count", short)]
    pub employees_count: bool,

    /// Current time relative to definite date.
    #[structopt(long = "current-time", short)]
    pub current_time: bool,

    /// Remove all conditions.
    #[structopt(long, short)]
    pub all: bool,

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
        if args.current_time || args.all {
            plan.conditions.current_time = None;
        }
        if args.employees_count || args.all {
            plan.conditions.employees_count = None;
        }
    }
    Ok(())
}
