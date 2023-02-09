pub use crate::shared::feed::*;
use crate::xml::get_element_with_text;
use crate::{WalmartResult, XmlSer};
use xml_builder::XMLElement;

#[derive(Debug, PartialEq)]
pub struct InventoryFeed {
  pub items: Vec<InventoryFeedItem>,
}

impl InventoryFeed {
  pub fn emit_xml(&self) -> WalmartResult<String> {
    self.to_string()
  }
}

#[derive(Debug, PartialEq)]
pub struct InventoryFeedItem {
  pub sku: String,
  pub quantity: i32,
}

impl XmlSer for InventoryFeed {
  fn to_xml(&self) -> WalmartResult<XMLElement> {
    let mut root = XMLElement::new("InventoryFeed");
    root.add_attribute("xmlns", "http://walmart.com/");

    let mut header = XMLElement::new("InventoryHeader");
    header.add_child(get_element_with_text("version", "1.4")?)?;
    root.add_child(header)?;

    for item in &self.items {
      let mut inventory = XMLElement::new("inventory");
      inventory.add_child(get_element_with_text("sku", &item.sku)?)?;

      let mut quantity = XMLElement::new("quantity");
      quantity.add_child(get_element_with_text("unit", "EACH")?)?;
      quantity.add_child(get_element_with_text("amount", item.quantity)?)?;

      inventory.add_child(quantity)?;

      root.add_child(inventory)?;
    }

    Ok(root)
  }
}
