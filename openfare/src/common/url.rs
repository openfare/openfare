#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Url {
    pub original: String,
    pub git: super::git::GitUrl,
}

impl std::str::FromStr for Url {
    type Err = anyhow::Error;
    fn from_str(url: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            original: url.to_string(),
            git: super::git::GitUrl::from_str(&url)?,
        })
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.original)
    }
}
