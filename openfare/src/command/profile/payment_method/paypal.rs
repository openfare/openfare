use crate::common::config::FileStore;
use anyhow::Result;
use openfare_lib::lock::payee::payment_methods::PaymentMethod;
use structopt::{self, StructOpt};

type Method = openfare_lib::lock::payee::payment_methods::PayPal;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct AddArguments {
    /// PayPal ID.
    #[structopt(long = "id")]
    pub id: Option<String>,

    /// Email.
    #[structopt(long = "email", required_unless = "id")]
    pub email: Option<String>,
}

pub fn add(args: &AddArguments) -> Result<()> {
    let payment_method = Method::new(&args.id, &args.email)?;
    let mut profile = crate::profile::Profile::load()?;
    (*profile).set_payment_method(
        &(Box::new(payment_method)
            as Box<dyn openfare_lib::lock::payee::payment_methods::PaymentMethod>),
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
    let mut profile = crate::profile::Profile::load()?;
    (*profile).remove_payment_method(&Method::associated_name())?;
    profile.dump()?;
    Ok(())
}
