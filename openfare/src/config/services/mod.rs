mod portal;

/// Payment services.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Services {
    pub portal: portal::Portal,
}
