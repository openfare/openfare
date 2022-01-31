use anyhow::{format_err, Context, Result};

use super::commands;
use super::common;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StaticData {
    pub name: String,
    pub registry_host_names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessExtension {
    process_path_: std::path::PathBuf,
    name_: String,
    registry_host_names_: Vec<String>,
}

impl common::FromProcess for ProcessExtension {
    fn from_process(
        process_path: &std::path::PathBuf,
        extension_config_path: &std::path::PathBuf,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let static_data: StaticData = if extension_config_path.is_file() {
            let file = std::fs::File::open(&extension_config_path)?;
            let reader = std::io::BufReader::new(file);
            serde_json::from_reader(reader)?
        } else {
            let static_data: Box<StaticData> = run_process(&process_path, &vec!["static-data"])?;
            let static_data = *static_data;

            let file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&extension_config_path)
                .context(format!(
                    "Can't open/create file for writing: {}",
                    extension_config_path.display()
                ))?;
            let writer = std::io::BufWriter::new(file);
            serde_json::to_writer(writer, &static_data)?;
            static_data
        };

        Ok(ProcessExtension {
            process_path_: process_path.clone(),
            name_: static_data.name,
            registry_host_names_: static_data.registry_host_names,
        })
    }
}

impl common::Extension for ProcessExtension {
    fn name(&self) -> String {
        self.name_.clone()
    }

    fn registries(&self) -> Vec<String> {
        self.registry_host_names_.clone()
    }

    fn package_dependencies_locks(
        &self,
        package_name: &str,
        package_version: &Option<&str>,
        extension_args: &Vec<String>,
    ) -> Result<commands::package_dependencies_locks::PackageDependenciesLocks> {
        let mut args = vec![
            super::commands::package_dependencies_locks::COMMAND_NAME,
            "--package-name",
            package_name,
        ];
        if let Some(package_version) = package_version {
            args.push("--package-version");
            args.push(package_version);
        }
        for extension_arg in extension_args {
            args.push("--extension-args");
            args.push(extension_arg);
        }
        let output: Box<commands::package_dependencies_locks::PackageDependenciesLocks> =
            run_process(&self.process_path_, &args)?;
        Ok(*output)
    }

    /// Returns a list of local package dependencies specification files.
    fn fs_defined_dependencies_locks(
        &self,
        working_directory: &std::path::PathBuf,
        extension_args: &Vec<String>,
    ) -> Result<commands::fs_defined_dependencies_locks::FsDefinedDependenciesLocks> {
        let working_directory = working_directory.to_str().ok_or(format_err!(
            "Failed to parse path into string: {}",
            working_directory.display()
        ))?;
        let mut args = vec![
            super::commands::fs_defined_dependencies_locks::COMMAND_NAME,
            "--working-directory",
            working_directory,
        ];
        for extension_arg in extension_args {
            args.push("--extension-args");
            args.push(extension_arg);
        }

        let output: Box<commands::fs_defined_dependencies_locks::FsDefinedDependenciesLocks> =
            run_process(&self.process_path_, &args)?;
        Ok(*output)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProcessResult<T> {
    pub ok: Option<T>,
    pub err: Option<String>,
}

fn run_process<'a, T: ?Sized>(process_path: &std::path::PathBuf, args: &Vec<&str>) -> Result<Box<T>>
where
    for<'de> T: serde::Deserialize<'de> + 'a,
{
    log::debug!(
        "Executing extensions process call with arguments\n{:?}",
        args
    );
    let process = process_path.to_str().ok_or(format_err!(
        "Failed to parse string from process path: {}",
        process_path.display()
    ))?;
    let handle = std::process::Command::new(process)
        .args(args)
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .output()?;

    let stdout = String::from_utf8_lossy(&handle.stdout);
    let stdout = stdout.to_string();

    let result = hex::decode(&stdout)?.clone();
    let process_result: ProcessResult<T> =
        bincode::deserialize(&result).expect("deserialize result with bincode");

    if let Some(result) = process_result.ok {
        Ok(Box::new(result))
    } else if let Some(result) = process_result.err {
        Err(format_err!(result))
    } else {
        Err(format_err!("Failed to find ok or err result from process."))
    }
}

#[test]
fn test_deserialize() -> Result<()> {
    let stdout = "0109000000000000006e706d6a732e636f6d01070000000000000069732d6576656e0500000000000000312e302e30000400000000000000090000000000000069732d6275666665720500000000000000312e312e3600090000000000000069732d6e756d6265720500000000000000332e302e3000060000000000000069732d6f64640500000000000000302e312e320007000000000000006b696e642d6f660500000000000000332e322e320000";
    let result = hex::decode(&stdout)?.clone();
    let process_result: ProcessResult<
        crate::extension::commands::package_dependencies_locks::PackageDependenciesLocks,
    > = bincode::deserialize(&result).expect("deserialize result with bincode");
    println!("process_result: {:?}", process_result);
    Ok(())
}
