use anyhow::Result;

pub fn run_command(
    args: Vec<&str>,
    working_directory: &std::path::PathBuf,
) -> Result<std::process::Output> {
    log::debug!(
        "Executing git command: git {args}\nWorking directory: {working_directory}",
        args = args.join(" ").to_string(),
        working_directory = working_directory.display()
    );
    let output = std::process::Command::new("git")
        .args(&args)
        .current_dir(working_directory)
        .output()?;
    log::debug!("Command execution complete: {:?}", output);

    if !output.status.success() {
        let args = args.join(" ").to_string();
        return Err(anyhow::format_err!(
            "Git command error: git {args}\n{status}",
            args = args,
            status = output.status
        ));
    }
    Ok(output)
}

pub fn commit(message: &str, working_directory: &std::path::PathBuf) -> Result<()> {
    let args = vec!["commit", "-am", message];
    if run_command(args, &working_directory).is_err() {
        log::debug!("Error encountered running git commit command. Possibly no change to commit.")
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitUrl {
    pub hostname: Option<String>,
    pub username: Option<String>,
    pub repository: Option<String>,
}

impl GitUrl {
    pub fn new(hostname: Option<&str>, username: Option<&str>, repository: Option<&str>) -> GitUrl {
        Self {
            hostname: hostname.map(String::from),
            username: username.map(String::from),
            repository: repository.map(String::from),
        }
    }

    pub fn as_ssh_url(&self) -> Option<String> {
        if let Some(hostname) = &self.hostname {
            if let Some(username) = &self.username {
                if let Some(repository) = &self.repository {
                    return Some(format!(
                        "git@{hostname}:{username}/{repository}.git",
                        hostname = hostname,
                        username = username,
                        repository = repository
                    ));
                }
            }
        }
        None
    }

    pub fn as_https_url(&self) -> Option<String> {
        if let Some(hostname) = &self.hostname {
            if let Some(username) = &self.username {
                if let Some(repository) = &self.repository {
                    return Some(format!(
                        "https://{hostname}/{username}/{repository}.git",
                        hostname = hostname,
                        username = username,
                        repository = repository
                    ));
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
                hostname: None,
                username: None,
                repository: None,
            })
        }
    }
}

fn parse_https_url(url: &str) -> Result<GitUrl> {
    let re = regex::Regex::new(
        r"(http(|s)://)?(?P<hostname>[^/]*)/(?P<username>[^/]*)(/(?P<repository>[^\.]*)(\.git)?)?$",
    )?;
    let captures = re.captures(url).ok_or(anyhow::format_err!(
        "Failed to capture regex groups: {url}",
        url = url
    ))?;
    let hostname = captures.name("hostname").map_or(None, |m| Some(m.as_str()));
    let username = captures.name("username").map_or(None, |m| Some(m.as_str()));
    let repository = captures
        .name("repository")
        .map_or(None, |m| Some(m.as_str()));

    // For GitHub or GitLab, if repository not given, assume same as username.
    let repository = if hostname == Some("github.com") || hostname == Some("gitlab.com") {
        repository.or(username)
    } else {
        repository
    };

    Ok(GitUrl::new(hostname, username, repository))
}

fn parse_git_url(url: &str) -> Result<GitUrl> {
    let re = regex::Regex::new(
        r"git@(?P<hostname>.*):(?P<username>.*)/(?P<repository>[^\.]*)(\.git)?$",
    )?;
    let captures = re.captures(url).ok_or(anyhow::format_err!(
        "Failed to capture regex groups: {url}",
        url = url
    ))?;
    let hostname = captures.name("hostname").map_or(None, |m| Some(m.as_str()));
    let username = captures.name("username").map_or(None, |m| Some(m.as_str()));
    let repository = captures
        .name("repository")
        .map_or(None, |m| Some(m.as_str()));

    Ok(GitUrl::new(hostname, username, repository))
}

fn is_https_git_url(url: &str) -> bool {
    !is_ssh_git_url(&url)
}

fn is_ssh_git_url(url: &str) -> bool {
    url.starts_with("git@")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_parse_https_url() -> Result<()> {
        let cases = vec![
            "https://github.com/rndhouse/rndhouse_repo.git",
            "http://gitlab.com/rndhouse/rndhouse_repo.git",
            "github.com/rndhouse/rndhouse_repo",
        ];
        let expected = vec![
            GitUrl::new(Some("github.com"), Some("rndhouse"), Some("rndhouse_repo")),
            GitUrl::new(Some("gitlab.com"), Some("rndhouse"), Some("rndhouse_repo")),
            GitUrl::new(Some("github.com"), Some("rndhouse"), Some("rndhouse_repo")),
        ];

        for (case, expect) in cases.iter().zip(expected.iter()) {
            let result = GitUrl::from_str(case)?;
            assert_eq!(&result, expect);
        }

        Ok(())
    }

    #[test]
    fn test_parse_github_gitlab_user_https_url() -> Result<()> {
        let cases = vec!["https://github.com/rndhouse", "http://gitlab.com/rndhouse"];
        let expected = vec![
            GitUrl::new(Some("github.com"), Some("rndhouse"), Some("rndhouse")),
            GitUrl::new(Some("gitlab.com"), Some("rndhouse"), Some("rndhouse")),
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
