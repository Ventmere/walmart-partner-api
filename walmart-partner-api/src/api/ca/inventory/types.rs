use xml_builder::XMLElement;

pub use crate::shared::inventory::*;
use crate::WalmartResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
  /// An arbitrary alphanumeric unique ID, seller-specified, identifying each item.
  /// Please note that get inventory returns nothing for sku field -_-
  #[serde(rename = "sku", default)]
  pub sku: String,
  #[serde(rename = "quantity")]
  pub quantity: InventoryQuantity,
  /// The number of days between when the item is ordered and when it is shipped
  #[serde(rename = "fulfillmentLagTime")]
  pub fulfillment_lag_time: i32,
  #[serde(rename = "partnerId", skip_serializing_if = "Option::is_none")]
  pub partner_id: Option<String>,
  #[serde(rename = "offerId", skip_serializing_if = "Option::is_none")]
  pub offer_id: Option<String>,
}

impl crate::XmlSer for Inventory {
  fn to_xml(&self) -> WalmartResult<XMLElement> {
    let mut inventory = XMLElement::new("inventory");
    inventory.add_attribute("xmlns", "http://walmart.com/");
    let mut sku = XMLElement::new("sku");
    sku.add_text(self.sku.clone())?;
    inventory.add_child(sku)?;

    inventory.add_child(self.quantity.to_xml()?)?;

    let mut fulfillment_lag_time = XMLElement::new("fulfillmentLagTime");
    fulfillment_lag_time.add_text(self.fulfillment_lag_time.to_string())?;
    inventory.add_child(fulfillment_lag_time)?;

    if let Some(partner_id_v) = &self.partner_id {
      let mut partner_id = XMLElement::new("partnerId");
      partner_id.add_text(partner_id_v.clone())?;
      inventory.add_child(partner_id)?;
    }
    if let Some(offer_id_v) = &self.offer_id {
      let mut offer_id = XMLElement::new("offerId");
      offer_id.add_text(offer_id_v.clone())?;
      inventory.add_child(offer_id)?;
    }
    Ok(inventory)
  }
}
