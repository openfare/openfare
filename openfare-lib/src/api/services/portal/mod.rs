pub mod basket;

lazy_static! {
    pub static ref ROUTE: String = format!("{}/portal", super::ROUTE.as_str());
}

pub type ApiKey = String;
