pub mod payees;
pub mod plans;

/// A software package's name and version.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}

/// A software package's OpenFare lock file (OPENFARE.lock).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Lock {
    pub plans: Vec<plans::PaymentPlan>,
    pub payees: std::collections::BTreeMap<payees::PayeeName, payees::Payee>,
}

pub type DependenciesLocks = std::collections::BTreeMap<Package, Option<Lock>>;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackageLocks {
    pub primary_package: Option<Package>,
    pub primary_package_lock: Option<Lock>,
    pub dependencies_locks: DependenciesLocks,
}

impl PackageLocks {
    pub fn has_locks(&self) -> bool {
        self.primary_package_lock.is_some() || !self.dependencies_locks.is_empty()
    }
}
