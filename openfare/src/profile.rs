use crate::common;
use anyhow::Result;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    #[serde(flatten)]
    profile: openfare_lib::profile::Profile,

    #[serde(skip)]
    pub from_url_status: Option<FromUrlStatus>,
}

#[derive(Debug, Clone)]
pub struct FromUrlStatus {
    // URL used in retrieving profile.
    pub url: crate::common::url::Url,

    // Method used to retrieve profile.
    pub method: FromUrlMethod,
}

#[derive(Debug, Clone)]
pub enum FromUrlMethod {
    Git,
    HttpGetJson,
}

impl Profile {
    pub fn from_url(url: &crate::common::url::Url) -> Result<Self> {
        match Self::from_http_get(&url) {
            Ok(profile) => Ok(profile),
            Err(_) => Self::from_git_url(&url),
        }
    }

    fn from_http_get(url: &crate::common::url::Url) -> Result<Self> {
        let client = reqwest::blocking::Client::new();

        // Convert github.com to raw content form.
        let url_str = url.to_string();
        let url_str = if url_str.contains("github.com") {
            url_str
                .replace("github.com", "raw.githubusercontent.com")
                .replace("/blob/", "/")
        } else {
            url_str
        };

        log::debug!("Sending HTTP GET request to endpoint: {}", url_str);
        let response = client.get(url_str).send()?;
        let profile = response.json::<openfare_lib::profile::Profile>()?;
        log::debug!("Response received.");

        Ok(Self {
            profile,
            from_url_status: Some(FromUrlStatus {
                url: url.clone(),
                method: FromUrlMethod::HttpGetJson,
            }),
        })
    }

    fn from_git_url(url: &crate::common::url::Url) -> Result<Self> {
        let tmp_dir = tempdir::TempDir::new("openfare_profile_from_git_url")?;
        let tmp_directory_path = tmp_dir.path().to_path_buf();

        let clone_url = if let Some(url) = url.git.as_ssh_url() {
            url
        } else {
            url.original.clone()
        };
        log::debug!("Attempting to clone repository using URL: {}", clone_url);
        let output = crate::common::git::run_command(
            vec!["clone", "--depth", "1", clone_url.as_str(), "."],
            &tmp_directory_path,
        )?;
        log::debug!("Clone output: {:?}", output);
        let path = tmp_directory_path.join(openfare_lib::profile::FILE_NAME);

        if !path.exists() {
            return Err(anyhow::format_err!(
                "Failed to find profile JSON in repository: {}",
                openfare_lib::profile::FILE_NAME
            ));
        }

        let file = std::fs::File::open(&path)?;
        let reader = std::io::BufReader::new(file);
        let profile: openfare_lib::profile::Profile = serde_json::from_reader(reader)?;
        Ok(Self {
            profile,
            from_url_status: Some(FromUrlStatus {
                url: url.clone(),
                method: FromUrlMethod::Git,
            }),
        })
    }
}

impl std::ops::Deref for Profile {
    type Target = openfare_lib::profile::Profile;

    fn deref(&self) -> &openfare_lib::profile::Profile {
        &self.profile
    }
}

impl std::ops::DerefMut for Profile {
    fn deref_mut(&mut self) -> &mut openfare_lib::profile::Profile {
        &mut self.profile
    }
}

impl common::config::FilePath for Profile {
    fn file_path() -> Result<std::path::PathBuf> {
        let paths = common::fs::ConfigPaths::new()?;
        Ok(paths.profile_file)
    }
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}
