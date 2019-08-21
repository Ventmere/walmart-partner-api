use crate::response::JsonMaybe;
use crate::result::*;
mod types;

pub use self::types::*;
use crate::client::{Client, Method, WalmartMarketplace};

impl Client {
  pub fn get_item_inventory(&self, sku: &str) -> WalmartResult<Inventory> {
    let path = match self.get_marketplace() {
      WalmartMarketplace::USA => "/v2/inventory",
      WalmartMarketplace::Canada => "/v3/inventory",
    };
    self
      .request_json(Method::GET, path, vec![("sku", sku)])?
      .send()?
      .json_maybe::<Inventory>()
      .map_err(Into::into)
  }

  pub fn update_item_inventory(&self, inventory: &Inventory) -> WalmartResult<Inventory> {
    let path = match self.get_marketplace() {
      WalmartMarketplace::USA => "/v2/inventory",
      WalmartMarketplace::Canada => "/v3/inventory",
    };
    self
      .request_json(Method::PUT, path, vec![("sku", &inventory.sku)])?
      .json(inventory)
      .send()?
      .json_maybe::<Inventory>()
      .map_err(Into::into)
  }
}
