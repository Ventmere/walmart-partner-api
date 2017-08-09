#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
  pub unit: String,
  pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Inventory {
  pub sku: String,
  pub quantity: Quantity,
  pub fulfillmentLagTime: i32,
}