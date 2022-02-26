#[macro_use]
extern crate lazy_static;

pub static HTTP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub mod api;
pub mod extension;
pub mod lock;
pub mod package;
pub mod price;
pub mod profile;
