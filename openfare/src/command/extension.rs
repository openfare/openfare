use anyhow::{format_err, Result};
use structopt::{self, StructOpt};

use crate::common;
use crate::extension;

#[derive(Debug, StructOpt, Clone)]
pub enum Subcommands {
    /// Add and enable extension.
    Add(AddArguments),

    /// Disable and delete extension.
    Remove(RemoveArguments),

    /// Enable extension.
    Enable(EnableArguments),

    /// Disable extension without deleting.
    Disable(DisableArguments),

    /// List installed extensions.
    List(ListArguments),
}

pub fn run_subcommand(subcommand: &Subcommands) -> Result<()> {
    match subcommand {
        Subcommands::Add(args) => {
            log::info!("Running command: extension add");
            add(&args)?;
        }
        Subcommands::Remove(args) => {
            log::info!("Running command: extension remove");
            remove(&args)?;
        }
        Subcommands::Enable(args) => {
            log::info!("Running command: extension enable");
            enable(&args)?;
        }
        Subcommands::Disable(args) => {
            log::info!("Running command: extension disable");
            disable(&args)?;
        }
        Subcommands::List(args) => {
            log::info!("Running command: extension list");
            list(&args)?;
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
pub struct AddArguments {
    /// Extension name, release archive URL, or GitHub repository URL.
    #[structopt(name = "name-or-url")]
    pub name_or_url: String,

    // Optional installation directory path.
    #[structopt(long = "install-directory", short = "d", name = "install-directory")]
    pub install_directory: Option<String>,
}

fn add(args: &AddArguments) -> Result<()> {
    log::info!("Adding extension using argument: {}", args.name_or_url);

    let bin_directory = match &args.install_directory {
        Some(install_directory) => {
            let path = shellexpand::full(&install_directory)?.to_string();
            std::path::PathBuf::from(&path)
        }
        None => common::fs::ensure_extensions_bin_directory()?.ok_or(format_err!(
            "Failed to find suitable directory for installing extension binary.\n\
                Please specify install directory with argument: --install-directory"
        ))?,
    };
    if !is_install_directory_discoverable(&bin_directory)? {
        println!(
            "WARNING: install directory is not the default \
            or not included in the PATH environment variable.\n\
            OpenFare may not be able to find the extension."
        )
    }
    log::info!("Using extension bin directory: {}", bin_directory.display());

    let extension_name = if args.name_or_url.contains("/") {
        log::debug!("Identified argument as URL.");
        let url = args.name_or_url.clone();
        if let Some(url) = try_parse_user_url(&url)? {
            log::debug!("Sanitized URL: {}", url);
            extension::manage::add_from_url(&url, &bin_directory)?
        } else {
            return Err(format_err!("Failed to parse URL: {}", url));
        }
    } else {
        log::debug!("Identified argument as name.");
        let name = extension::manage::clean_name(&args.name_or_url);
        let url = get_url_from_name(&name)?;
        extension::manage::add_from_url(&url, &bin_directory)?
    };

    let mut config = common::config::Config::load()?;
    extension::manage::update_config(&mut config)?;

    println!("Added extension: {}", extension_name);
    Ok(())
}

/// Returns true if OpenFare can discover extensions stored in given directory.
fn is_install_directory_discoverable(directory: &std::path::PathBuf) -> Result<bool> {
    if is_directory_in_path_env(&directory)? {
        Ok(true)
    } else {
        match common::fs::get_extensions_default_directory() {
            Some(default_directory) => Ok(&default_directory == directory),
            None => Ok(false),
        }
    }
}

fn is_directory_in_path_env(directory: &std::path::PathBuf) -> Result<bool> {
    let env_path_value =
        std::env::var_os("PATH").ok_or(format_err!("Failed to read PATH environment variable."))?;
    let paths = std::env::split_paths(&env_path_value);

    Ok(paths.into_iter().any(|path| path == *directory))
}

fn try_parse_user_url(url: &str) -> Result<Option<url::Url>> {
    let url = if !url.starts_with("https://") && !url.starts_with("http://") {
        String::from("https://") + url
    } else {
        url.to_string()
    };
    let url = match url.strip_suffix(".git") {
        Some(url) => url.into(),
        None => url,
    };
    Ok(url::Url::parse(&url).ok())
}

fn get_url_from_name(name: &str) -> Result<url::Url> {
    Ok(url::Url::parse(
        format!("https://github.com/openfare/openfare-{name}", name = name).as_str(),
    )?)
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct RemoveArguments {
    /// Extension name.
    pub name: String,
}

fn remove(args: &RemoveArguments) -> Result<()> {
    let mut config = common::config::Config::load()?;
    extension::manage::update_config(&mut config)?;

    let name = extension::manage::clean_name(&args.name);
    extension::manage::remove(&name)?;
    println!("Removed extension: {}", name);
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct EnableArguments {
    /// Extension name.
    pub name: String,
}

fn enable(args: &EnableArguments) -> Result<()> {
    let mut config = common::config::Config::load()?;
    extension::manage::update_config(&mut config)?;

    let name = extension::manage::clean_name(&args.name);
    let all_extension_names = extension::manage::get_all_names(&config)?;
    if !all_extension_names.contains(&name) {
        return Err(format_err!(
            "Failed to find extension. Known extensions: {}",
            all_extension_names
                .into_iter()
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    extension::manage::enable(&name, &mut config)?;
    println!("Enabled extension: {}", name);
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct DisableArguments {
    /// Extension name.
    pub name: String,
}

fn disable(args: &DisableArguments) -> Result<()> {
    let mut config = common::config::Config::load()?;
    extension::manage::update_config(&mut config)?;

    let name = extension::manage::clean_name(&args.name);
    let all_extension_names = extension::manage::get_all_names(&config)?;
    if !all_extension_names.contains(&name) {
        return Err(format_err!(
            "Failed to find extension. Known extensions: {}",
            all_extension_names
                .into_iter()
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    extension::manage::disable(&name, &mut config)?;
    println!("Disabled extension: {}", name);
    Ok(())
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[structopt::clap::AppSettings::DisableVersion]
)]
pub struct ListArguments {}

fn list(_args: &ListArguments) -> Result<()> {
    let mut config = common::config::Config::load()?;
    extension::manage::update_config(&mut config)?;
    for name in extension::manage::get_all_names(&config)? {
        println!("{}", name);
    }
    Ok(())
}
