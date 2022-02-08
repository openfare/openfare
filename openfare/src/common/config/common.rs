use anyhow::{format_err, Context, Result};

pub fn bool_from_string(value: &str) -> Result<bool> {
    Ok(match value {
        "true" => true,
        "false" => false,
        _ => {
            return Err(format_err!(
                "Expected value: `true` or `false`. Found: {}",
                value
            ));
        }
    })
}

pub trait FilePath {
    fn file_path() -> Result<std::path::PathBuf>;
}

pub trait FileStore: FilePath {
    fn load() -> Result<Self>
    where
        Self: Sized;
    fn dump(&mut self) -> Result<()>;
}

impl<'de, T> FileStore for T
where
    T: FilePath + Default + serde::de::DeserializeOwned + serde::Serialize,
{
    fn load() -> Result<Self> {
        if !Self::file_path()?.is_file() {
            let mut default = Self::default();
            default.dump()?;
        }

        let file = std::fs::File::open(Self::file_path()?)?;
        let reader = std::io::BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    fn dump(&mut self) -> Result<()> {
        if Self::file_path()?.is_file() {
            std::fs::remove_file(&Self::file_path()?)?;
        }

        let file = std::fs::OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(&Self::file_path()?)
            .context(format!(
                "Can't open/create file for writing: {}",
                Self::file_path()?.display()
            ))?;

        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }
}

fn get_regex(command: &str) -> Result<regex::Regex> {
    Ok(regex::Regex::new(
        format!("{}(\\.(.*))?", command).as_str(),
    )?)
}

pub fn get_field(name_arg: &str, command: &str, error_message: &str) -> Result<String> {
    let re = get_regex(&command)?;
    let captures = re
        .captures(name_arg)
        .ok_or(format_err!(error_message.to_string()))?;

    let field = if let Some(field) = captures.get(2) {
        field.as_str().to_string()
    } else if let Some(field) = captures.get(0) {
        field.as_str().to_string()
    } else {
        return Err(format_err!(error_message.to_string()));
    };
    Ok(field)
}

pub fn is_match(name: &str, command: &str) -> Result<bool> {
    Ok(get_regex(&command)?.is_match(name))
}
