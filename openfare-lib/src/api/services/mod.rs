pub mod basket;
pub mod portal;

lazy_static! {
    pub static ref ROUTE: String = format!("{}/services", super::ROUTE.as_str());
}
