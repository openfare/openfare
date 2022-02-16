pub type Name = String;
pub type Payees = std::collections::BTreeMap<Name, Payee>;
pub type PaymentMethodName = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payee {
    pub url: Option<String>,
    #[serde(flatten)]
    pub profile: crate::profile::Profile,
}

pub fn get_lock_payee(
    profile: &crate::profile::Profile,
    all_lock_payees: &std::collections::BTreeMap<Name, Payee>,
) -> Option<(Name, Payee)> {
    for (name, existing_payee) in all_lock_payees {
        if profile.unique_id == existing_payee.profile.unique_id {
            return Some((name.clone(), existing_payee.clone()));
        }
    }
    None
}

// TODO: name -> label
pub fn unique_name(payee_name: &Name, payee: &Payee) -> Name {
    let unique_id = payee.profile.unique_id.to_string()[..13].to_string();
    format!("{payee_name}___{unique_id}")
}
