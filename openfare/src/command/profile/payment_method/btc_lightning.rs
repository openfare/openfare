use crate::common::config::FileStore;
use anyhow::Result;
use structopt::{self, StructOpt};

type Method = openfare_lib::profile::payment_methods::BtcLightning;
const METHOD_TYPE: openfare_lib::profile::payment_methods::Methods =
    openfare_lib::profile::payment_methods::Methods::BtcLightning;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct AddArguments {
    /// Public key destination
    #[structopt(name = "keysend")]
    pub keysend: String,
}

pub fn add(args: &AddArguments) -> Result<()> {
    let payment_method = Method::new(&args.keysend)?;
    let mut profile = crate::handles::ProfileHandle::load()?;
    (*profile).set_payment_method(
        &(Box::new(payment_method)
            as Box<dyn openfare_lib::profile::payment_methods::PaymentMethod>),
    )?;
    profile.dump()?;
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct RemoveArguments {}

pub fn remove(_args: &RemoveArguments) -> Result<()> {
    let mut profile = crate::handles::ProfileHandle::load()?;
    (*profile).remove_payment_method(&METHOD_TYPE)?;
    profile.dump()?;
    Ok(())
}