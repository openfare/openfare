use anyhow::Result;
use structopt::{self, StructOpt};

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct Arguments {
    /// Config setting field name.
    pub name: Option<String>,

    /// Config setting field value.
    pub value: Option<String>,
}

pub fn run_command(args: &Arguments) -> Result<()> {
    let mut config = crate::common::config::Config::load()?;
    if let Some(name) = &args.name {
        if let Some(value) = &args.value {
            config.set(&name, &value)?;
            config.dump()?;
            println!("set {name}: {value}", name = name, value = value);
        } else {
            println!("{}", config.get(&name)?);
        }
    } else {
        println!("{}", config);
    }
    Ok(())
}
