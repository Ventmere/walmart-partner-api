use error::*;
use serde_qs;
mod types;

pub use self::types::*;
use client::{Method, Client};
use feed::FeedAck;

pub trait InventoryApi {
  fn get_item_inventory(&self, sku: &str) -> Result<Inventory>;
  fn update_item_inventory(&self, inventory: &Inventory) -> Result<Inventory>;
  fn bulk_update(&self, inventory_list: &Vec<Inventory>) -> Result<FeedAck>; 
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
    unimplemented!()
  }

  fn bulk_update(&self, inventory_list: &Vec<Inventory>) -> Result<FeedAck> {
    unimplemented!()
  }
}