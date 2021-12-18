#[cfg(feature = "prod")]
#[path ="prod/temperature.rs"]
pub mod temperature;

#[cfg(feature = "sim")]
#[path = "dev/temperature.rs"]
pub mod temperature;
