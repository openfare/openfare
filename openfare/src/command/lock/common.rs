use anyhow::{format_err, Context, Result};
use serde::Serialize;
use std::io::Write;
use structopt::{self, StructOpt};

#[derive(Debug)]
pub struct LockFileHandle {
    pub lock: openfare_lib::lock::Lock,
    path: std::path::PathBuf,
}

impl LockFileHandle {
    pub fn new(user_lock_file_path: &Option<std::path::PathBuf>, force: bool) -> Result<Self> {
        let path = if let Some(user_lock_file_path) = user_lock_file_path {
            user_lock_file_path.clone()
        } else {
            let working_directory = std::env::current_dir()?;
            working_directory.join(openfare_lib::lock::FILE_NAME)
        };

        if path.exists() && !force {
            return Err(format_err!(
                "File already exists and --force flag not given. Exiting.\n{}",
                path.display()
            ));
        }

        Ok(Self {
            lock: openfare_lib::lock::Lock::default(),
            path,
        })
    }

    pub fn load(user_lock_file_path: &Option<std::path::PathBuf>) -> Result<Self> {
        let handle = if let Some(user_lock_file_path) = user_lock_file_path {
            Self::get_lock_from_user_provided_path(&user_lock_file_path)?
        } else {
            // No user path given, search for lock file.
            if let Some(path) = find_lock_file()? {
                Self {
                    lock: from_file(&path)?,
                    path,
                }
            } else {
                return Err(format_err!(
                    "Filed to find lock file. Provide path or change working directory."
                ));
            }
        };
        Ok(handle)
    }

    pub fn path(&self) -> &std::path::PathBuf {
        &self.path
    }

    fn get_lock_from_user_provided_path(path: &std::path::PathBuf) -> Result<Self> {
        Ok(if path.is_file() {
            Self {
                lock: from_file(&path)?,
                path: path.clone(),
            }
        } else if path.exists() {
            return Err(format_err!(
                "Failed to interpret path {} as file.",
                path.display()
            ));
        } else {
            // User provided path does not exist.
            Self {
                lock: openfare_lib::lock::Lock::default(),
                path: path.clone(),
            }
        })
    }
}

impl Drop for LockFileHandle {
    fn drop(&mut self) {
        // TODO: Check if lock has been modified using hashes. Don't write unmodified.
        if self.path.is_file() {
            std::fs::remove_file(&self.path).unwrap_or_default();
        }

        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut serializer = serde_json::Serializer::with_formatter(buf, formatter);
        self.lock.serialize(&mut serializer).unwrap();

        let lock_json = String::from_utf8(serializer.into_inner()).unwrap();

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(&self.path)
            .context(format!(
                "Can't open/create file for writing: {}",
                self.path.display()
            ))
            .unwrap();

        file.write_all(lock_json.as_bytes())
            .expect("Unable to write data");

        // let writer = std::io::BufWriter::new(file);
        // serde_json::to_writer_pretty(writer, &self.lock).unwrap();
    }
}

fn find_lock_file() -> Result<Option<std::path::PathBuf>> {
    let working_directory = std::env::current_dir()?;

    assert!(working_directory.is_absolute());
    let mut working_directory = working_directory.clone();

    loop {
        let target_absolute_path = working_directory.join(openfare_lib::lock::FILE_NAME);
        if target_absolute_path.is_file() {
            return Ok(Some(target_absolute_path));
        }

        // No need to move further up the directory tree after this loop.
        if working_directory == std::path::PathBuf::from("/") {
            break;
        }

        // Move further up the directory tree.
        working_directory.pop();
    }
    Ok(None)
}

fn from_file(path: &std::path::PathBuf) -> Result<openfare_lib::lock::Lock> {
    let file = std::fs::File::open(&path)?;
    let reader = std::io::BufReader::new(file);
    let lock: openfare_lib::lock::Lock = serde_json::from_reader(reader)?;
    Ok(lock)
}

#[derive(Debug, StructOpt, Clone)]
pub struct LockFilePathArg {
    /// Lock file path. Searches in current working directory if not given.
    #[structopt(name = "lock-file-path")]
    pub path: Option<std::path::PathBuf>,
}
