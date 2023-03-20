use chrono::{DateTime, Utc};
use xml_builder::XMLElement;

use crate::{WalmartResult, XmlSer};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderListMeta {
  /// Total no of purchase orders.
  #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
  pub total_count: Option<i32>,
  /// Number of purchase orders in the current page.
  #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
  pub limit: Option<i32>,
  /// String to be used as query parameter for getting next set of purchase orders, when more than 200 orders are retrieved.
  #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
  pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShippingInfo {
  /// The customer's phone number
  #[serde(rename = "phone")]
  pub phone: String,
  /// The estimated time and date for the delivery of the item. Format: yyyy-MM-ddThh:MM:ssZ Example: '2020-06-15T06:00:00Z'
  #[serde(rename = "estimatedDeliveryDate")]
  pub estimated_delivery_date: DateTime<Utc>,
  /// The estimated time and date when the item will be shipped. Format: yyyy-MM-ddThh:MM:ssZ Example: '2020-06-15T06:00:00Z'
  #[serde(rename = "estimatedShipDate")]
  pub estimated_ship_date: DateTime<Utc>,
  /// The shipping method. Can be one of the following: Standard, Express, OneDay, WhiteGlove, Value or Freight
  #[serde(rename = "methodCode")]
  pub method_code: String,
  #[serde(rename = "postalAddress")]
  pub postal_address: PostalAddress,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostalAddress {
  /// The name for the person/place of shipping address
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLineItem {
  /// The name of the product associated with the line item. Example: 'Kenmore CF1' or '2086883 Canister Secondary Filter Generic 2 Pack'
  #[serde(rename = "productName")]
  pub product_name: String,
  /// An arbitrary alphanumeric unique ID, assigned to each item in the XSD file.
  #[serde(rename = "sku")]
  pub sku: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLineCharges {
  /// List of elements that make up a charge
  #[serde(rename = "charge")]
  #[serde(default)]
  pub charge: Vec<OrderLineCharge>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLineCharge {
  /// The charge type for line items can be one of the following: PRODUCT or SHIPPING For details, refer to 'Charge Types'
  #[serde(rename = "chargeType")]
  pub charge_type: String,
  /// If chargeType is PRODUCT, chargeName is Item Price. If chargeType is SHIPPING, chargeName is Shipping
  #[serde(rename = "chargeName")]
  pub charge_name: String,
  #[serde(rename = "chargeAmount")]
  pub charge_amount: CurrencyAmount,
  #[serde(rename = "tax", skip_serializing_if = "Option::is_none")]
  pub tax: Option<Tax>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrencyAmount {
  /// The type of currency for the charge. Example: USD for US Dollars
  #[serde(rename = "currency")]
  pub currency: String,
  /// The numerical amount for that charge. Example: 9.99
  #[serde(rename = "amount")]
  pub amount: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tax {
  /// The name associated with the tax. Example: 'Sales Tax'
  #[serde(rename = "taxName")]
  pub tax_name: String,
  #[serde(rename = "taxAmount")]
  pub tax_amount: CurrencyAmount,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLineStatusQuantity {
  /// Always use 'EACH'
  #[serde(rename = "unitOfMeasurement")]
  pub unit_of_measurement: String,
  /// Always use '1'
  #[serde(rename = "amount")]
  pub amount: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLineStatuses {
  /// Details about the Order Line status
  #[serde(rename = "orderLineStatus")]
  #[serde(default)]
  pub order_line_status: Vec<OrderLineStatus>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLineStatus {
  /// Should be 'Created'
  #[serde(rename = "status")]
  pub status: String,
  #[serde(rename = "statusQuantity")]
  pub status_quantity: OrderLineStatusQuantity,
  /// If order is cancelled, cancellationReason will explain the reason
  #[serde(rename = "cancellationReason", skip_serializing_if = "Option::is_none")]
  pub cancellation_reason: Option<String>,
  #[serde(rename = "trackingInfo", skip_serializing_if = "Option::is_none")]
  pub tracking_info: Option<OrderLineTrackingInfo>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLineTrackingInfo {
  /// The date the package was shipped
  #[serde(rename = "shipDateTime")]
  pub ship_date_time: DateTime<Utc>,
  #[serde(rename = "carrierName")]
  pub carrier_name: OrderLineTrackingCarrier,
  /// The shipping method. Can be one of the following: Standard, Express, Oneday, or Freight
  #[serde(rename = "methodCode")]
  pub method_code: String,
  /// The shipment tracking number
  #[serde(rename = "trackingNumber")]
  pub tracking_number: String,
  /// The URL for tracking the shipment
  #[serde(rename = "trackingURL", skip_serializing_if = "Option::is_none")]
  pub tracking_url: Option<String>,
}

impl XmlSer for OrderLineTrackingInfo {
  fn to_xml(&self) -> WalmartResult<XMLElement> {
    let mut tracking_info = XMLElement::new("trackingInfo");
    let mut ship_date_time = XMLElement::new("shipDateTime");
    ship_date_time.add_text(self.ship_date_time.to_rfc3339())?;
    tracking_info.add_child(ship_date_time)?;

    tracking_info.add_child(self.carrier_name.to_xml()?)?;

    let mut method_code = XMLElement::new("methodCode");
    method_code.add_text(self.method_code.clone())?;
    tracking_info.add_child(method_code)?;

    let mut tracking_number = XMLElement::new("trackingNumber");
    tracking_number.add_text(self.tracking_number.clone())?;
    tracking_info.add_child(tracking_number)?;

    if let Some(tracking_url_v) = self.tracking_url.clone() {
      let mut tracking_url = XMLElement::new("trackingURL");
      tracking_url.add_text(tracking_url_v)?;
      tracking_info.add_child(tracking_url)?;
    }

    Ok(tracking_info)
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLineTrackingCarrier {
  /// Other carrier name
  #[serde(rename = "otherCarrier", skip_serializing_if = "Option::is_none")]
  pub other_carrier: Option<String>,
  /// The package shipment carrier
  #[serde(rename = "carrier", skip_serializing_if = "Option::is_none")]
  pub carrier: Option<String>,
}

impl XmlSer for OrderLineTrackingCarrier {
  fn to_xml(&self) -> WalmartResult<XMLElement> {
    let mut carrier_name = XMLElement::new("carrierName");
    if let Some(other_carrier_v) = self.other_carrier.clone() {
      let mut other_carrier = XMLElement::new("otherCarrier");
      other_carrier.add_text(other_carrier_v)?;
      carrier_name.add_child(other_carrier)?;
    }

    if let Some(carrier_v) = self.carrier.clone() {
      let mut carrier = XMLElement::new("carrier");
      carrier.add_text(carrier_v)?;
      carrier_name.add_child(carrier)?;
    }
    Ok(carrier_name)
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderRefund {
  #[serde(rename = "refundId", skip_serializing_if = "Option::is_none")]
  pub refund_id: Option<String>,
  #[serde(rename = "refundComments", skip_serializing_if = "Option::is_none")]
  pub refund_comments: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderFulfillment {
  #[serde(rename = "fulfillmentOption", skip_serializing_if = "Option::is_none")]
  pub fulfillment_option: Option<String>,
  #[serde(rename = "shipMethod", skip_serializing_if = "Option::is_none")]
  pub ship_method: Option<String>,
  #[serde(rename = "storeId", skip_serializing_if = "Option::is_none")]
  pub store_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShipOrderLineAsn {
  pub package_asn: String,
  pub pallet_asn: Option<String>,
}

impl XmlSer for ShipOrderLineAsn {
  fn to_xml(&self) -> WalmartResult<XMLElement> {
    let mut asn = XMLElement::new("asn");

    let mut package_asn = XMLElement::new("packageASN");
    package_asn.add_text(self.package_asn.clone())?;
    asn.add_child(package_asn)?;

    if let Some(pallet_asn_v) = self.pallet_asn.clone() {
      let mut pallet_asn = XMLElement::new("palletASN");
      pallet_asn.add_text(pallet_asn_v)?;
      asn.add_child(pallet_asn)?;
    }
    Ok(asn)
  }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ShipOrderLineStatusQuantity {
  pub unit_of_measurement: Option<String>,
  pub amount: Option<String>,
}

impl XmlSer for Option<ShipOrderLineStatusQuantity> {
  fn to_xml(&self) -> WalmartResult<XMLElement> {
    let mut status_quantity = XMLElement::new("statusQuantity");
    let mut unit_of_measurement = XMLElement::new("unitOfMeasurement");
    let unit_of_measurement_v = self
      .as_ref()
      .and_then(|v| v.unit_of_measurement.clone())
      .unwrap_or("EACH".to_string());
    unit_of_measurement.add_text(unit_of_measurement_v)?;
    status_quantity.add_child(unit_of_measurement)?;

    let mut amount = XMLElement::new("amount");
    let amount_v = self
      .as_ref()
      .and_then(|v| v.amount.clone())
      .unwrap_or("1".to_string());
    amount.add_text(amount_v)?;
    status_quantity.add_child(amount)?;

    Ok(status_quantity)
  }
}
