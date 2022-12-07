pub use types::*;

use crate::api::us::Client;
use crate::client::Method;
use crate::result::WalmartResult;

mod types;

impl Client {
  pub async fn get_item_inventory(&self, sku: impl AsRef<str>) -> WalmartResult<Inventory> {
    let req = self.req_xml(Method::GET, "/v3/inventory", vec![("sku", sku)])?;
    self.send(req).await?.res_xml().await
  }

  pub async fn update_item_inventory(&self, inventory: Inventory) -> WalmartResult<Inventory> {
    let req = self
      .req_xml(Method::PUT, "/v3/inventory", vec![("sku", &inventory.sku)])?
      .body_xml(inventory)?;
    self.send(req).await?.res_xml().await
  }
}

#[cfg(test)]
mod tests {
  use mockito::mock;

  use crate::test_util::get_client_us;

  use super::*;

  #[tokio::test]
  async fn test_get_item_inventory() {
    let client = get_client_us();
    let _m = mock("GET", "/v3/inventory?sku=97964_KFTest")
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(
        r##"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<inventory xmlns="http://walmart.com/">
    <sku>97964_KFTest</sku>
    <quantity>
        <unit>EACH</unit>
        <amount>10</amount>
    </quantity>
</inventory>
"##,
      )
      .create();

    let inventory = client.get_item_inventory("97964_KFTest").await.unwrap();
    assert_eq!(inventory.sku, "97964_KFTest");
    assert_eq!(inventory.quantity.amount, 10);
  }

  #[tokio::test]
  async fn test_update_item_inventory() {
    let client = get_client_us();
    let _m = mock("PUT", "/v3/inventory?sku=97964_KFTest")
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(
        r##"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<inventory xmlns="http://walmart.com/">
    <sku>97964_KFTest</sku>
    <quantity>
        <unit>EACH</unit>
        <amount>3</amount>
    </quantity>
</inventory>"##,
      )
      .create();

    let body = Inventory {
      sku: "97964_KFTest".to_string(),
      quantity: InventoryQuantity {
        unit: "EACH".to_string(),
        amount: 3,
      },
    };

    let inventory = client.update_item_inventory(body).await.unwrap();
    assert_eq!(inventory.sku, "97964_KFTest");
    assert_eq!(inventory.quantity.amount, 3);
  }
}
