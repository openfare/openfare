pub struct ExtensionLocks {
    pub extension_name: String,
    pub package_locks:
        std::collections::BTreeMap<openfare_lib::package::Package, openfare_lib::lock::Lock>,
}
