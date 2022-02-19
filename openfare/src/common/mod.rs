pub mod config;
pub mod fs;
pub mod git;
pub mod url;

pub static HTTP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
