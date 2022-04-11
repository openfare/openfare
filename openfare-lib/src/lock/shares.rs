use super::payee;

pub type Shares = std::collections::BTreeMap<payee::Label, u64>;
