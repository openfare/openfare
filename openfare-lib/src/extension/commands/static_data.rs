use super::common;
use crate::extension::common::Extension;
use anyhow::Result;

pub fn run_command<T: Extension + std::fmt::Debug>(extension: &T) -> Result<()> {
    let data = Ok(crate::extension::process::StaticData {
        name: extension.name(),
        registry_host_names: extension.registries(),
    });
    common::communicate_result(data)?;
    Ok(())
}
