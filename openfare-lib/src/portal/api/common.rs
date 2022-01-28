pub type ApiKey = String;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Package {
    pub registry: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Plan {
    pub label: String,
    pub digest: String,
}
