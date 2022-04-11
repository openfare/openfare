pub type ExtensionName = String;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Item {
    pub package: crate::package::Package,
    pub extension_name: ExtensionName,
    pub plans: std::collections::BTreeMap<String, crate::lock::plan::Plan>,
    pub total_price: crate::price::Price,
    pub payees: crate::lock::payee::Payees,
    pub shares: Option<crate::lock::shares::Shares>,
}

impl Item {
    // TODO: replace valid_payees(.) with lock.validate(.)
    /// Returns valid payees.
    ///
    /// A payee is valid if it has allocated shares or if shares are undefined.
    pub fn valid_payees(&self) -> crate::lock::payee::Payees {
        if let Some(shares) = &self.shares {
            let payee_labels = shares
                .iter()
                .map(|(label, _)| label)
                .collect::<std::collections::BTreeSet<_>>();
            let payees = self
                .payees
                .iter()
                .filter(|(label, _payee)| payee_labels.contains(label))
                .map(|(label, payee)| (label.clone(), payee.clone()))
                .collect();
            payees
        } else {
            // If shares unspecified, all payees are applicable.
            self.payees.clone()
        }
    }
}
