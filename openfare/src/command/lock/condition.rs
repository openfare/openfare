use anyhow::Result;
use structopt::{self, StructOpt};

use super::common;

#[derive(Debug, StructOpt, Clone)]
pub struct ConditionArguments {
    /// For profit organization.
    #[structopt(long = "for-profit")]
    pub for_profit: bool,

    /// Expiration date. Example: "2022-01-31"
    #[structopt(long)]
    pub expiration: Option<String>,

    /// Number of employees in the organization. Example: "> 100"
    #[structopt(name = "employees-count", long = "employees-count")]
    pub employees_count: Option<String>,
}

impl std::convert::TryInto<openfare_lib::lock::plan::conditions::Conditions>
    for ConditionArguments
{
    type Error = anyhow::Error;
    fn try_into(self) -> Result<openfare_lib::lock::plan::conditions::Conditions, Self::Error> {
        let for_profit = if self.for_profit {
            Some(openfare_lib::lock::plan::conditions::ForProfit::new())
        } else {
            None
        };
        let expiration = if let Some(expiration) = &self.expiration {
            Some(openfare_lib::lock::plan::conditions::Expiration::try_from(
                expiration.as_str(),
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
            for_profit,
            expiration,
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
    let mut lock_handle = crate::handles::LockHandle::load(&args.lock_file_args.path)?;
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

    /// For profit organization.
    #[structopt(long = "for-profit")]
    pub for_profit: bool,

    /// Number of employees in the organization.
    #[structopt(long = "employees-count")]
    pub employees_count: bool,

    /// Expiration date.
    #[structopt(long)]
    pub expiration: bool,

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
    let mut lock_handle = crate::handles::LockHandle::load(&args.lock_file_args.path)?;
    for (_plan_id, plan) in lock_handle
        .lock
        .plans
        .iter_mut()
        .filter(|(id, _plan)| plan_ids.contains(id.as_str()) || plan_ids.is_empty())
    {
        if args.for_profit || args.all {
            plan.conditions.for_profit = None;
        }
        if args.expiration || args.all {
            plan.conditions.expiration = None;
        }
        if args.employees_count || args.all {
            plan.conditions.employees_count = None;
        }
    }
    Ok(())
}
