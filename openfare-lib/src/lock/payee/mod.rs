pub type Label = String;
pub type Payees = std::collections::BTreeMap<Label, Payee>;
pub type PaymentMethodName = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payee {
    pub url: Option<String>,
    #[serde(flatten)]
    pub profile: crate::profile::Profile,
}

pub fn get_lock_payee(
    profile: &crate::profile::Profile,
    all_lock_payees: &std::collections::BTreeMap<Label, Payee>,
) -> Option<(Label, Payee)> {
    for (name, existing_payee) in all_lock_payees {
        if profile.unique_id == existing_payee.profile.unique_id {
            return Some((name.clone(), existing_payee.clone()));
        }
    }
    None
}

pub fn unique_label(payee_label: &Label, payee: &Payee) -> Label {
    let unique_id = payee.profile.unique_id.to_string()[..13].to_string();
    format!("{payee_label}___{unique_id}")
}
