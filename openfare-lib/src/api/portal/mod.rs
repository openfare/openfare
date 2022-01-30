pub mod checkout;
pub mod common;

lazy_static! {
    pub static ref ROUTE: String = format!("{}/portal", super::ROUTE.as_str());
}
