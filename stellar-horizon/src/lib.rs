extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate stellar_base;

#[macro_use]
pub mod request;

pub mod api;
pub mod client;
pub mod error;
pub mod resources;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
