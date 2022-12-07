#[macro_use]
extern crate serde_derive;

pub use api::*;
pub use client::{WalmartCredential, WalmartMarketplace};
pub use result::{WalmartError, WalmartResult};

mod api;
mod client;
pub mod result;
mod shared;
mod sign;
#[cfg(test)]
mod test_util;
mod utils;

pub trait XmlSer {
  fn to_xml(&self) -> WalmartResult<xml_builder::XMLElement>;
}
