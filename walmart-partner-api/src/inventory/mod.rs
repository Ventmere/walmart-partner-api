use crate::response::JsonMaybe;
use crate::result::*;
mod types;

pub use self::types::*;
use crate::client::{Client, Method};

impl Client {
  pub fn get_item_inventory(&self, sku: &str) -> WalmartResult<Inventory> {
    let path = "/v3/inventory";
    self
      .send(self.request_json(Method::GET, path, vec![("sku", sku)])?)?
      .json_maybe::<Inventory>()
      .map_err(Into::into)
  }

  pub fn update_item_inventory(&self, inventory: &Inventory) -> WalmartResult<Inventory> {
    let path = "/v3/inventory";
    self
      .send(
        self
          .request_json(Method::PUT, path, vec![("sku", &inventory.sku)])?
          .json(inventory),
      )?
      .json_maybe::<Inventory>()
      .map_err(Into::into)
  }
}
