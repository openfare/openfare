use crate::lock;

/// A software package's name and version.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Package {
    pub registry: String,
    pub name: String,
    pub version: String,
}

pub type DependenciesLocks = std::collections::BTreeMap<Package, Option<lock::Lock>>;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackageLocks {
    pub primary_package: Option<Package>,
    pub primary_package_lock: Option<lock::Lock>,
    pub dependencies_locks: DependenciesLocks,
}

impl PackageLocks {
    pub fn has_locks(&self) -> bool {
        self.primary_package_lock.is_some() || !self.dependencies_locks.is_empty()
    }

    // Returns a map of packages to their total set of plan conditions.
    pub fn package_conditions(
        &self,
    ) -> std::collections::BTreeMap<Package, Vec<Box<dyn lock::plan::conditions::Condition>>> {
        let mut packages_conditions = std::collections::BTreeMap::new();
        for (package, lock) in &self.dependencies_locks {
            if let Some(lock) = lock {
                let mut conditions = vec![];
                for (_id, plan) in &lock.plans {
                    conditions.extend(plan.conditions.as_vec());
                }
                packages_conditions.insert(package.clone(), conditions);
            }
        }
        packages_conditions
    }
}
