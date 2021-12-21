use super::common;
use crate::extension::common::Extension;
use anyhow::Result;
use structopt::{self, StructOpt};

pub const COMMAND_NAME: &str = "fs-defined-dependencies-configs";

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
#[structopt(global_setting = structopt::clap::AppSettings::TrailingVarArg)]
pub struct Arguments {
    /// Working directory.
    #[structopt(name = "working-directory", long)]
    pub working_directory: String,

    #[structopt(name = "extension-args", long)]
    pub extension_args: Vec<String>,
}

pub fn run_command<T: Extension + std::fmt::Debug>(args: &Arguments, extension: &T) -> Result<()> {
    let working_directory = std::path::PathBuf::from(&args.working_directory);
    let result =
        extension.fs_defined_dependencies_configs(&working_directory, &args.extension_args);
    common::communicate_result(result)?;
    Ok(())
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FsDefinedDependenciesConfigs {
    pub project_path: std::path::PathBuf,
    pub package_configs: crate::package::PackageConfigs,
}
