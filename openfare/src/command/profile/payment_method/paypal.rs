use crate::common::fs::FileStore;
use anyhow::Result;
use structopt::{self, StructOpt};

type Method = openfare_lib::profile::payment_methods::PayPal;
const METHOD_TYPE: openfare_lib::profile::payment_methods::Methods =
    openfare_lib::profile::payment_methods::Methods::PayPal;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct AddArguments {
    /// PayPal ID.
    #[structopt(long)]
    pub id: Option<String>,

    /// Email.
    #[structopt(long, required_unless = "id")]
    pub email: Option<String>,
}

pub fn add(args: &AddArguments) -> Result<()> {
    let payment_method = Method::new(&args.id, &args.email)?;
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
