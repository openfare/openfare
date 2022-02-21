mod portal;

/// Payment services.
#[derive(
    Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct Services {
    pub portal: portal::Portal,
}
