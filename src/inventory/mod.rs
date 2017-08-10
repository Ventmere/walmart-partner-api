use error::*;
mod types;

pub use self::types::*;
use client::{Method, Client};

pub trait InventoryApi {
  fn get_item_inventory(&self, sku: &str) -> Result<Inventory>;
  fn update_item_inventory(&self, inventory: &Inventory) -> Result<Inventory>;
}

impl InventoryApi for Client {
  fn get_item_inventory(&self, sku: &str) -> Result<Inventory> {
    self.request_json(Method::Get, "/v2/inventory", vec![
      ("sku", sku)
    ])?
      .send()?
      .json::<Inventory>()
      .map_err(Into::into)
  }

  fn update_item_inventory(&self, inventory: &Inventory) -> Result<Inventory> {
    self.request_json(Method::Put, "/v2/inventory", vec![
      ("sku", &inventory.sku)
    ])?
      .json(inventory)?
      .send()?
      .json::<Inventory>()
      .map_err(Into::into)
  }
}
