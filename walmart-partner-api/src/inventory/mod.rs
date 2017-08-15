use error::*;
use response::JsonMaybe;
mod types;

pub use self::types::*;
use client::{Method, Client};

impl Client {
  pub fn get_item_inventory(&self, sku: &str) -> Result<Inventory> {
    self.request_json(Method::Get, "/v2/inventory", vec![
      ("sku", sku)
    ])?
      .send()?
      .json_maybe::<Inventory>()
      .map_err(Into::into)
  }

  pub fn update_item_inventory(&self, inventory: &Inventory) -> Result<Inventory> {
    self.request_json(Method::Put, "/v2/inventory", vec![
      ("sku", &inventory.sku)
    ])?
      .json(inventory)?
      .send()?
      .json_maybe::<Inventory>()
      .map_err(Into::into)
  }
}
