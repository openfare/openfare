use crate::common::fs::FileStore;
use anyhow::Result;
use structopt::{self, StructOpt};

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct AddArguments {
    /// Secret API key. Found here: https://cloud.lnpay.co/developers/dashboard
    pub secret_api_key: String,

    /// Set payment method as default.
    #[structopt(long, short)]
    pub default: bool,
}

pub fn add(args: &AddArguments) -> Result<()> {
    let mut config = crate::config::Config::load()?;
    config.services.lnpay = Some(crate::config::services::lnpay::LnPay {
        secret_api_key: args.secret_api_key.clone(),
    });
    config.dump()?;
    println!("Added service: LNPAY (https://lnpay.co)");
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
    let mut config = crate::config::Config::load()?;
    if config.services.default == crate::services::Service::LnPay {
        config.services.default = crate::services::Service::Portal;
    }
    config.services.lnpay = None;
    config.dump()?;
    Ok(())
}
