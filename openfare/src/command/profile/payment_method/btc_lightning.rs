use crate::common::fs::FileStore;
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
    /// Wallet LNURL. Example: lnurl1dp69e...
    pub lnurl: Option<String>,
    /// Optional fallback: keysend node public key.
    pub keysend: Option<String>,
    /// Setup using payment service.
    #[structopt(long, short, required_unless = "lnurl")]
    pub service: Option<crate::services::Service>,
}

pub fn add(args: &AddArguments) -> Result<()> {
    let config = crate::config::Config::load()?;

    let lnurl = if let Some(lnurl) = &args.lnurl {
        lnurl.clone()
    } else {
        if let Some(service) = &args.service {
            crate::services::lnurl_receive_address(&service, &config)?
                .ok_or(anyhow::format_err!("Service failed to derive LNURL."))?
        } else {
            return Err(anyhow::format_err!(
                "Service must be specified if LNURL not given."
            ));
        }
    };

    let payment_method = Method::new(&lnurl, &args.keysend)?;
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
