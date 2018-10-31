use walmart_partner_api::inventory::*;
use walmart_partner_api::Client;

pub fn set_inventory(client: &Client, sku: &str, quantity: i32, lagtime: i32) {
  let inventory = Inventory {
    quantity: Quantity {
      unit: "EACH".to_string(),
      amount: quantity,
    },
    sku: sku.to_string(),
    fulfillmentLagTime: lagtime,
  };
  let res = client.update_item_inventory(&inventory).unwrap();
  println!("{}", serde_json::to_string_pretty(&res).unwrap());
}
