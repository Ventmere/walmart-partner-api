pub use types::*;

use crate::api::ca::Client;
use crate::client::Method;
use crate::result::WalmartResult;

mod types;

impl Client {
  pub async fn get_item_inventory(&self, sku: impl AsRef<str>) -> WalmartResult<Inventory> {
    let req = self.req_xml(Method::GET, "/v3/ca/inventory", vec![("sku", sku)])?;
    self.send(req).await?.res_xml().await
  }

  pub async fn update_item_inventory(&self, inventory: Inventory) -> WalmartResult<Inventory> {
    let req = self
      .req_xml(
        Method::PUT,
        "/v3/ca/inventory",
        vec![("sku", &inventory.sku)],
      )?
      .body_xml(inventory)?;
    self.send(req).await?.res_xml().await
  }
}

#[cfg(test)]
mod tests {
  use mockito::mock;

  use crate::test_util::get_client_ca;

  use super::*;

  #[tokio::test]
  async fn test_get_item_inventory() {
    let client = get_client_ca();
    let _m = mock("GET", "/v3/ca/inventory?sku=1068155")
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(
        r##"
<?xml version="1.0" encoding="UTF-8"?>
<inventory xmlns:ns2="http://walmart.com/">
    <sku>1068155</sku>
    <quantity>
        <unit>EACH</unit>
        <amount>23</amount>
    </quantity>
    <fulfillmentLagTime>1</fulfillmentLagTime>
</inventory>
"##,
      )
      .create();

    let inventory = client.get_item_inventory("1068155").await.unwrap();
    assert_eq!(inventory.sku, "1068155");
    assert_eq!(inventory.quantity.amount, 23);
  }

  #[tokio::test]
  async fn test_update_item_inventory() {
    let client = get_client_ca();
    let _m = mock("PUT", "/v3/ca/inventory?sku=1068155")
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(
        r##"
<?xml version="1.0" encoding="UTF-8"?>
<inventory>
    <sku>1068155</sku>
    <quantity>
        <unit>EACH</unit>
        <amount>23</amount>
    </quantity>
    <fulfillmentLagTime>1</fulfillmentLagTime>
</inventory>"##,
      )
      .create();

    let body = Inventory {
      sku: "1068155".to_string(),
      quantity: InventoryQuantity {
        unit: "EACH".to_string(),
        amount: 23,
      },
      fulfillment_lag_time: 1,
      partner_id: None,
      offer_id: None,
    };

    let inventory = client.update_item_inventory(body).await.unwrap();
    assert_eq!(inventory.sku, "1068155");
    assert_eq!(inventory.quantity.amount, 23);
  }
}
