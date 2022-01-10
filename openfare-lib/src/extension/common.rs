use anyhow::Result;

#[derive(
    Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct VersionError(String);

impl VersionError {
    pub fn from_missing_version() -> Self {
        Self("Missing version number".to_string())
    }

    pub fn from_parse_error(raw_version_number: &str) -> Self {
        Self(format!("Version parse error: {}", raw_version_number))
    }

    pub fn message(&self) -> String {
        self.0.clone()
    }
}

pub type VersionParseResult = std::result::Result<String, VersionError>;

/// A dependency as specified within a dependencies definition file.
#[derive(
    Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct Dependency {
    pub name: String,
    pub version: VersionParseResult,
}

pub trait DependenciesCollection: Sized {
    fn registry_host_name(&self) -> &String;
    fn dependencies(&self) -> &Vec<Dependency>;
}

/// Package dependencies found by querying a registry.
#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PackageDependencies {
    // Package version (included incase version not given as an argument).
    pub package_version: VersionParseResult,

    /// Dependencies registry host name.
    pub registry_host_name: String,

    /// Dependencies specified within the dependencies specification file.
    pub dependencies: Vec<Dependency>,
}

impl DependenciesCollection for PackageDependencies {
    fn registry_host_name(&self) -> &String {
        &self.registry_host_name
    }
    fn dependencies(&self) -> &Vec<Dependency> {
        &self.dependencies
    }
}

/// A dependencies specification file found from inspecting the local filesystem.
#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FileDefinedDependencies {
    /// Absolute file path for dependencies specification file.
    pub path: std::path::PathBuf,

    /// Dependencies registry host name.
    pub registry_host_name: String,

    /// Dependencies specified within the dependencies specification file.
    pub dependencies: Vec<Dependency>,
}

impl DependenciesCollection for FileDefinedDependencies {
    fn registry_host_name(&self) -> &String {
        &self.registry_host_name
    }
    fn dependencies(&self) -> &Vec<Dependency> {
        &self.dependencies
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegistryPackageMetadata {
    pub registry_host_name: String,
    pub human_url: String,
    pub artifact_url: String,
    // True if this registry is the primary registry, otherwise false.
    pub is_primary: bool,
    // Included here incase package version was not given but found.
    pub package_version: String,
}

pub trait FromLib: Extension + Send + Sync {
    /// Initialize extension from a library.
    fn new() -> Self
    where
        Self: Sized;
}

pub trait FromProcess: Extension + Send + Sync {
    /// Initialize extension from a process.
    fn from_process(
        process_path: &std::path::PathBuf,
        extension_config_path: &std::path::PathBuf,
    ) -> Result<Self>
    where
        Self: Sized;
}

pub trait Extension: Send + Sync {
    // Returns extension short name.
    fn name(&self) -> String;

    // Returns supported registries host names.
    fn registries(&self) -> Vec<String>;

    /// Returns OpenFare locks for a package and its dependencies.
    fn package_dependencies_locks(
        &self,
        package_name: &str,
        package_version: &Option<&str>,
        extension_args: &Vec<String>,
    ) -> Result<super::commands::package_dependencies_locks::PackageDependenciesLocks>;

    /// Return OpenFare locks for a local project's dependencies.
    fn fs_defined_dependencies_locks(
        &self,
        working_directory: &std::path::PathBuf,
        extension_args: &Vec<String>,
    ) -> Result<super::commands::fs_defined_dependencies_locks::FsDefinedDependenciesLocks>;
}
