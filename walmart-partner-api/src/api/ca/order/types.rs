use chrono::{DateTime, Utc};

pub use crate::shared::order::*;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderList {
  #[serde(rename = "meta")]
  pub meta: OrderListMeta,
  #[serde(rename = "elements")]
  pub elements: Orders,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Orders {
  /// Purchase Order List
  #[serde(rename = "order")]
  #[serde(default)]
  pub order: Vec<Order>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
  /// A unique ID associated with the seller's purchase order
  #[serde(rename = "purchaseOrderId")]
  pub purchase_order_id: String,
  /// A unique ID associated with the sales order for specified customer
  #[serde(rename = "customerOrderId")]
  pub customer_order_id: String,
  /// The email address of the customer for the sales order
  #[serde(rename = "customerEmailId")]
  pub customer_email_id: String,
  /// The date the customer submitted the sales order
  #[serde(rename = "orderDate")]
  pub order_date: DateTime<Utc>,
  #[serde(rename = "shippingInfo")]
  pub shipping_info: ShippingInfo,
  #[serde(rename = "orderLines")]
  pub order_lines: OrderLines,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLines {
  /// A list of order lines in the order
  #[serde(rename = "orderLine")]
  #[serde(default)]
  pub order_line: Vec<OrderLine>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLine {
  /// The line number associated with the details for each individual item in the purchase order
  #[serde(rename = "lineNumber")]
  pub line_number: String,
  /// The ship from country is associated with the details for each individual item in the purchase order
  #[serde(rename = "shipFromCountry", skip_serializing_if = "Option::is_none")]
  pub ship_from_country: Option<String>,
  #[serde(rename = "item")]
  pub item: OrderLineItem,
  #[serde(rename = "charges")]
  pub charges: OrderLineCharges,
  #[serde(rename = "orderLineQuantity")]
  pub order_line_quantity: OrderLineStatusQuantity,
  /// The date shown on the recent order status
  #[serde(rename = "statusDate")]
  pub status_date: DateTime<Utc>,
  #[serde(rename = "orderLineStatuses")]
  pub order_line_statuses: OrderLineStatuses,
  #[serde(rename = "refund", skip_serializing_if = "Option::is_none")]
  pub refund: Option<OrderRefund>,
  #[serde(
    rename = "originalCarrierMethod",
    skip_serializing_if = "Option::is_none"
  )]
  pub original_carrier_method: Option<String>,
  #[serde(rename = "referenceLineId", skip_serializing_if = "Option::is_none")]
  pub reference_line_id: Option<String>,
  #[serde(rename = "fulfillment", skip_serializing_if = "Option::is_none")]
  pub fulfillment: Option<OrderFulfillment>,
  #[serde(rename = "intentToCancel", skip_serializing_if = "Option::is_none")]
  pub intent_to_cancel: Option<String>,
  #[serde(rename = "configId", skip_serializing_if = "Option::is_none")]
  pub config_id: Option<String>,
}
