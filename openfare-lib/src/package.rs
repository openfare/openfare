use crate::lock;

/// A software package's name and version.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Package {
    pub registry: String,
    pub name: String,
    pub version: String,
}

pub type DependenciesLocks = std::collections::BTreeMap<Package, Option<lock::Lock>>;

// TODO: Add 'Result' version of PackageLocks for extension lock file errors.

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

    // Returns a unique vector of conditions metadata from locks.
    pub fn conditions_metadata(&self) -> Vec<Box<dyn lock::plan::conditions::ConditionMetadata>> {
        let mut result = vec![];

        let mut handle_lock = |lock: &lock::Lock| {
            for (_id, plan) in &lock.plans {
                for metadata in plan.conditions.metadata() {
                    if !result.iter().any(
                        |m: &Box<dyn lock::plan::conditions::ConditionMetadata>| {
                            *m.name() == *metadata.name()
                        },
                    ) {
                        result.push(metadata);
                    }
                }
            }
        };

        if let Some(lock) = &self.primary_package_lock {
            handle_lock(&lock);
        }
        for (_package, lock) in &self.dependencies_locks {
            if let Some(lock) = lock {
                handle_lock(&lock);
            }
        }

        result
    }

    /// Filter for valid dependencies locks.
    pub fn filter_valid_dependencies_locks(&self) -> Self {
        let mut result = self.clone();
        for (package, lock) in &self.dependencies_locks {
            if let Some(lock) = lock {
                if lock.plans.is_empty() {
                    continue;
                }
                result
                    .dependencies_locks
                    .insert(package.clone(), Some(lock.clone()));
            }
        }
        result
    }
}
