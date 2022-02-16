use anyhow::Result;

pub fn run_command(
    args: Vec<&str>,
    working_directory: &std::path::PathBuf,
) -> Result<std::process::Output> {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(working_directory)
        .output()?;
    Ok(output)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitUrl {
    pub original_url: String,
    pub hostname: Option<String>,
    pub username: Option<String>,
    pub repository: Option<String>,
}

impl GitUrl {
    pub fn new(
        original_url_: &str,
        hostname: Option<&str>,
        username: Option<&str>,
        repository: Option<&str>,
    ) -> GitUrl {
        Self {
            original_url: original_url_.to_string(),
            hostname: hostname.map(String::from),
            username: username.map(String::from),
            repository: repository.map(String::from),
        }
    }

    pub fn as_ssh_url(&self) -> Option<String> {
        if let Some(hostname) = &self.hostname {
            if let Some(username) = &self.username {
                if let Some(repository) = &self.repository {
                    return Some(format!("git@{hostname}:{username}/{repository}.git"));
                }
            }
        }
        None
    }

    pub fn as_https_url(&self) -> Option<String> {
        if let Some(hostname) = &self.hostname {
            if let Some(username) = &self.username {
                if let Some(repository) = &self.repository {
                    return Some(format!("https://{hostname}/{username}/{repository}.git"));
                }
            }
        }
        None
    }
}

impl std::str::FromStr for GitUrl {
    type Err = anyhow::Error;
    fn from_str(url: &str) -> Result<Self, Self::Err> {
        if is_https_git_url(&url) {
            parse_https_url(&url)
        } else if is_ssh_git_url(&url) {
            parse_git_url(&url)
        } else {
            Ok(Self {
                original_url: url.to_string(),
                hostname: None,
                username: None,
                repository: None,
            })
        }
    }
}

fn parse_https_url(url: &str) -> Result<GitUrl> {
    let re = regex::Regex::new(
        r"http(|s)://(?P<hostname>.*)/(?P<username>.*)/(?P<repository>[^\.]*)(\.git)?$",
    )?;
    let captures = re
        .captures(url)
        .ok_or(anyhow::format_err!("Failed to capture regex groups: {url}"))?;
    let hostname = captures.name("hostname").map_or(None, |m| Some(m.as_str()));
    let username = captures.name("username").map_or(None, |m| Some(m.as_str()));
    let repository = captures
        .name("repository")
        .map_or(None, |m| Some(m.as_str()));

    Ok(GitUrl::new(url, hostname, username, repository))
}

fn parse_git_url(url: &str) -> Result<GitUrl> {
    let re = regex::Regex::new(
        r"git@(?P<hostname>.*):(?P<username>.*)/(?P<repository>[^\.]*)(\.git)?$",
    )?;
    let captures = re
        .captures(url)
        .ok_or(anyhow::format_err!("Failed to capture regex groups: {url}"))?;
    let hostname = captures.name("hostname").map_or(None, |m| Some(m.as_str()));
    let username = captures.name("username").map_or(None, |m| Some(m.as_str()));
    let repository = captures
        .name("repository")
        .map_or(None, |m| Some(m.as_str()));

    Ok(GitUrl::new(url, hostname, username, repository))
}

fn is_https_git_url(url: &str) -> bool {
    !is_ssh_git_url(&url)
}

fn is_ssh_git_url(url: &str) -> bool {
    url.starts_with("git@")
}

mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_parse_https_url() -> Result<()> {
        let cases = vec![
            "https://github.com/rndhouse/rndhouse_repo.git",
            "http://gitlab.com/rndhouse/rndhouse_repo.git",
            "http://github.com/rndhouse/rndhouse_repo",
        ];
        let expected = vec![
            GitUrl::new(
                "https://github.com/rndhouse/rndhouse_repo.git",
                Some("github.com"),
                Some("rndhouse"),
                Some("rndhouse_repo"),
            ),
            GitUrl::new(
                "http://gitlab.com/rndhouse/rndhouse_repo.git",
                Some("gitlab.com"),
                Some("rndhouse"),
                Some("rndhouse_repo"),
            ),
            GitUrl::new(
                "http://github.com/rndhouse/rndhouse_repo",
                Some("github.com"),
                Some("rndhouse"),
                Some("rndhouse_repo"),
            ),
        ];

        for (case, expect) in cases.iter().zip(expected.iter()) {
            let result = GitUrl::from_str(case)?;
            assert_eq!(&result, expect);
        }

        Ok(())
    }

    #[test]
    fn test_parse_ssh_url() -> Result<()> {
        let cases = vec!["git@github.com:rndhouse/rndhouse_repo.git"];
        let expected = vec![GitUrl::new(
            "git@github.com:rndhouse/rndhouse_repo.git",
            Some("github.com"),
            Some("rndhouse"),
            Some("rndhouse_repo"),
        )];

        for (case, expect) in cases.iter().zip(expected.iter()) {
            let result = GitUrl::from_str(case)?;
            assert_eq!(&result, expect);
        }

        Ok(())
    }
}