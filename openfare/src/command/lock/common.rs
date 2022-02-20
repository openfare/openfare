use structopt::{self, StructOpt};

#[derive(Debug, StructOpt, Clone)]
pub struct LockFilePathArg {
    /// Lock file path. Searches in current working directory if not given.
    #[structopt(name = "lock-file-path", long, short = "p")]
    pub path: Option<std::path::PathBuf>,
}
