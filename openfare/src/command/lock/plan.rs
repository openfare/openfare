use super::common;
use anyhow::Result;
use structopt::{self, StructOpt};

#[derive(Debug, StructOpt, Clone)]

pub enum AddArguments {
    /// Add plan for compulsory fees.
    Compulsory(AddCompulsoryArguments),

    /// Add plan for voluntary donations.
    Voluntary(AddVoluntaryArguments),
}

pub fn add(subcommand: &AddArguments) -> Result<()> {
    match subcommand {
        AddArguments::Compulsory(args) => {
            add_compulsory(&args)?;
        }
        AddArguments::Voluntary(args) => {
            add_voluntary(&args)?;
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
pub struct AddCompulsoryArguments {
    /// Payment plan price. Example: "50.2USD"
    #[structopt(long, short)]
    pub price: String,

    #[structopt(flatten)]
    pub conditions: super::condition::ConditionArguments,

    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

pub fn add_compulsory(args: &AddCompulsoryArguments) -> Result<()> {
    let mut lock_handle = crate::handles::LockHandle::load(&args.lock_file_args.path)?;
    let id = openfare_lib::lock::plan::next_id(&lock_handle.lock.plans)?;

    let conditions = args.conditions.clone().try_into()?;

    let plan = openfare_lib::lock::plan::Plan {
        r#type: openfare_lib::lock::plan::PlanType::Compulsory,
        conditions,
        price: Some(args.price.parse().expect("parse price")),
    };
    lock_handle.lock.plans.insert(id.clone(), plan.clone());

    println!(
        "{}",
        serde_json::to_string_pretty(&maplit::btreemap! {id.clone() => plan.clone()})?
    );
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct AddVoluntaryArguments {
    #[structopt(flatten)]
    pub lock_file_args: common::LockFilePathArg,
}

pub fn add_voluntary(args: &AddVoluntaryArguments) -> Result<()> {
    let mut lock_handle = crate::handles::LockHandle::load(&args.lock_file_args.path)?;
    let id = openfare_lib::lock::plan::next_id(&lock_handle.lock.plans)?;
    let plan = openfare_lib::lock::plan::Plan {
        r#type: openfare_lib::lock::plan::PlanType::Voluntary,
        conditions: openfare_lib::lock::plan::conditions::Conditions::default(),
        price: None,
    };
    lock_handle.lock.plans.insert(id.clone(), plan.clone());

    println!(
        "{}",
        serde_json::to_string_pretty(&maplit::btreemap! {id.clone() => plan.clone()})?
    );
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
    pub id: Vec<u64>,
}

pub fn remove(args: &RemoveArguments) -> Result<()> {
    let mut lock_handle = crate::handles::LockHandle::load(&None)?;
    let ids = if args.id.is_empty() {
        lock_handle
            .lock
            .plans
            .iter()
            .map(|(plan_id, _)| plan_id.clone())
            .collect::<Vec<_>>()
    } else {
        args.id.iter().map(|id| id.to_string()).collect::<Vec<_>>()
    };

    // Ensure all IDs present for atomic removal.
    for id in &ids {
        if !lock_handle.lock.plans.contains_key(id) {
            println!("Failed to find plan: {}", id);
        }
    }
    for id in &ids {
        lock_handle.lock.plans.remove(id);
    }

    if !ids.is_empty() {
        println!("Plans removed (ID): {}", ids.join(", "));
    }
    Ok(())
}
