use crate::common::fs::FileStore;
use anyhow::Result;
use structopt::{self, StructOpt};

use crate::extensions;
mod dependencies;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct Arguments {
    /// Donation. Example: 1usd or 200sats
    pub donation: Option<openfare_lib::price::Price>,

    /// Specify payment service.
    #[structopt(long, short)]
    pub service: Option<crate::services::Service>,

    /// Set specified payment service as default.
    #[structopt(long, short)]
    pub default: bool,

    /// Specify an extension for handling the package and its dependencies.
    /// Example values: py, js, rs
    #[structopt(long = "extension", short = "e", name = "name")]
    pub extension_names: Option<Vec<String>>,
}

pub fn run_command(args: &Arguments, extension_args: &Vec<String>) -> Result<()> {
    let mut config = crate::config::Config::load()?;
    extensions::manage::update_config(&mut config)?;
    if args.default {
        if let Some(service) = &args.service {
            config.services.default = service.clone();
            config.dump()?;
        }
    }
    let config = config;

    let extensions = extensions::manage::from_names_arg(&args.extension_names, &config)?;
    let all_extension_locks = dependencies::get_locks(&extensions, &extension_args, &config)?;
    let mut items = vec![];
    for extension_locks in all_extension_locks {
        let packages_plans = dependencies::get_packages_plans(
            &extension_locks.extension_name,
            &extension_locks.package_locks,
            &config,
        )?;
        items.extend(packages_plans);
    }
    crate::services::pay(&args.donation, &items, &args.service, &config)?;
    Ok(())
}
