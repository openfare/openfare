pub mod plans;

/// A software package's name and version.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}

/// A software package's OpenFare config file (OPENFARE.json).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub plans: Vec<plans::PaymentPlan>,
}

pub type DependenciesConfigs = std::collections::BTreeMap<Package, Option<Config>>;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackageConfigs {
    pub primary_package: Option<Package>,
    pub primary_package_config: Option<Config>,
    pub dependencies_configs: DependenciesConfigs,
}

impl PackageConfigs {
    pub fn has_configs(&self) -> bool {
        self.primary_package_config.is_some() || !self.dependencies_configs.is_empty()
    }
}
