use super::common;
use crate::extension::common::Extension;
use anyhow::Result;
use structopt::{self, StructOpt};

pub const COMMAND_NAME: &str = "package-dependencies-configs";

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
#[structopt(global_setting = structopt::clap::AppSettings::TrailingVarArg)]
pub struct Arguments {
    /// Package name.
    #[structopt(name = "package-name", long)]
    pub package_name: String,

    /// Package version.
    #[structopt(name = "package-version", long)]
    pub package_version: Option<String>,

    #[structopt(name = "extension-args", long)]
    pub extension_args: Vec<String>,
}

pub fn run_command<T: Extension + std::fmt::Debug>(args: &Arguments, extension: &T) -> Result<()> {
    let result = extension.package_dependencies_configs(
        &args.package_name,
        &args.package_version.as_deref(),
        &args.extension_args,
    );
    common::communicate_result(result)?;
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackageDependenciesConfigs {
    pub registry_host_name: String,
    pub package_configs: crate::package::PackageConfigs,
}
