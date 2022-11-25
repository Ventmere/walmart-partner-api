use serde::{Deserialize, Serialize};

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

impl Inventory {
  pub fn new(sku: &str, quantity: i32, fulfillment_lag_time: i32) -> Inventory {
    Inventory {
      sku: sku.to_owned(),
      quantity: Quantity {
        unit: "EACH".to_owned(),
        amount: quantity,
      },
      fulfillmentLagTime: fulfillment_lag_time,
    }
  }
}
