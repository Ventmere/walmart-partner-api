use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyAmount {
  pub currency: String,
  pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostalAddress {
  /// The name for the shipment
  #[serde(rename = "name")]
  pub name: String,
  /// The first line of the shipping address
  #[serde(rename = "address1")]
  pub address1: String,
  /// The second line of the shipping address
  #[serde(rename = "address2", skip_serializing_if = "Option::is_none")]
  pub address2: Option<String>,
  /// The city of the shipping address
  #[serde(rename = "city")]
  pub city: String,
  /// The state of the shipping address
  #[serde(rename = "state")]
  pub state: String,
  /// The zip code of the shipping address
  #[serde(rename = "postalCode")]
  pub postal_code: String,
  /// The country of the shipping address
  #[serde(rename = "country")]
  pub country: String,
  /// The address type, example: 'RESIDENTIAL'
  #[serde(rename = "addressType", skip_serializing_if = "Option::is_none")]
  pub address_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingInformation {
  /// The customer's phone number
  #[serde(rename = "phone")]
  pub phone: String,
  /// The estimated time and date for the delivery of the item. Format: yyyy-MM-ddThh:MM:ssZ Example: '2016-06-15T06:00:00Z'
  #[serde(rename = "estimatedDeliveryDate")]
  pub estimated_delivery_date: DateTime<Utc>,
  /// The estimated time and date when the item will be shipped. Format: yyyy-MM-ddThh:MM:ssZ Example: '2016-06-15T06:00:00Z'
  #[serde(rename = "estimatedShipDate")]
  pub estimated_ship_date: DateTime<Utc>,
  /// The shipping method. Can be one of the following: Standard, Express, Oneday, or Freight
  #[serde(rename = "methodCode")]
  pub method_code: String,
  #[serde(rename = "postalAddress")]
  pub postal_address: PostalAddress,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLineItem {
  pub productName: String,
  pub sku: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tax {
  pub taxName: String,
  pub taxAmount: CurrencyAmount,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLineChargeItem {
  pub chargeType: String,
  pub chargeName: String,
  pub chargeAmount: CurrencyAmount,
  pub tax: Option<Tax>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
  pub unitOfMeasurement: String,
  pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLineCharges {
  pub charge: Vec<OrderLineChargeItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLineTrackingInfoCarrier {
  pub otherCarrier: Option<String>,
  pub carrier: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLineTrackingInfo {
  pub shipDateTime: i64,
  pub carrierName: OrderLineTrackingInfoCarrier,
  pub methodCode: String,
  pub trackingNumber: String,
  pub trackingURL: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLineStatus {
  pub status: String,
  pub statusQuantity: Quantity,
  // pub cancellationReason: Option<?>,
  pub trackingInfo: Option<OrderLineTrackingInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLineStatuses {
  pub order_line_status: Vec<OrderLineStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLine {
  /// The line number associated with the details for each individual item in the purchase order
  #[serde(rename = "lineNumber")]
  pub line_number: String,
  pub item: OrderLineItem,
  pub charges: OrderLineCharges,
  pub order_line_quantity: Quantity,
  pub status_date: DateTime<Utc>,
  pub order_line_statuses: OrderLineStatuses,
  pub ship_from_country: Option<String>,
  // pub refund: Option<?>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLines {
  #[serde(rename = "orderLine", skip_serializing_if = "Option::is_none")]
  pub order_line: Option<Vec<OrderLine>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
  pub shipping_info: ShippingInformation,
  #[serde(rename = "orderLines")]
  pub order_lines: OrderLines,
}
