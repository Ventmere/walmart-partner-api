use xml_builder::XMLElement;

pub use crate::shared::inventory::*;
use crate::WalmartResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "inventory")]
pub struct Inventory {
  /// A seller-provided Product ID. Response will have decoded value.
  #[serde(rename = "sku")]
  pub sku: String,
  #[serde(rename = "quantity")]
  pub quantity: InventoryQuantity,
}

impl crate::XmlSer for Inventory {
  fn to_xml(&self) -> WalmartResult<XMLElement> {
    let mut inventory = XMLElement::new("inventory");
    inventory.add_attribute("xmlns", "http://walmart.com/");

    let mut sku = XMLElement::new("sku");
    sku.add_text(self.sku.clone())?;
    inventory.add_child(sku)?;
    inventory.add_child(self.quantity.to_xml()?)?;

    Ok(inventory)
  }
}
