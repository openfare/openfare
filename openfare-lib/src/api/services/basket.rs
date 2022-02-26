pub type ExtensionName = String;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Item {
    pub package: crate::package::Package,
    pub extension_name: ExtensionName,
    pub plans: std::collections::BTreeMap<String, crate::lock::plan::Plan>,
    pub total_price: crate::price::Price,
    pub payees: crate::lock::payee::Payees,
}

impl Item {
    pub fn plan_payees(&self, plan_id: &str) -> Option<crate::lock::payee::Payees> {
        let plan = self.plans.get(plan_id);
        if let Some(plan) = plan {
            if let Some(shares) = &plan.payments.shares {
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
                Some(payees)
            } else {
                // If shares unspecified, all payees are applicable.
                Some(self.payees.clone())
            }
        } else {
            None
        }
    }
}
