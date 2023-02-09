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
  pub fulfillment_lag_time: i32,
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
      inventory.add_child(get_element_with_text(
        "fulfillmentLagTime",
        item.fulfillment_lag_time,
      )?)?;

      root.add_child(inventory)?;
    }

    Ok(root)
  }
}

#[test]
fn test_emit_inventory_feed() {
  let data = InventoryFeed {
    items: vec![
      InventoryFeedItem {
        sku: "1068155".to_string(),
        quantity: 10,
        fulfillment_lag_time: 2,
      },
      InventoryFeedItem {
        sku: "10210321".to_string(),
        quantity: 20,
        fulfillment_lag_time: 2,
      },
    ],
  };
  let xml = data.emit_xml().unwrap();
  let want = r#"<?xml version="1.0" encoding="UTF-8"?>
  <InventoryFeed xmlns="http://walmart.com/">
    <InventoryHeader>
        <version>1.4</version>
    </InventoryHeader>
    <inventory>
        <sku>1068155</sku>
        <quantity>
            <unit>EACH</unit>
            <amount>10</amount>
        </quantity>
         <fulfillmentLagTime>2</fulfillmentLagTime>
    </inventory>
    <inventory>
        <sku>10210321</sku>
        <quantity>
            <unit>EACH</unit>
            <amount>20</amount>
        </quantity>
         <fulfillmentLagTime>2</fulfillmentLagTime>
    </inventory>
</InventoryFeed>"#;
  crate::test_util::assert_xml_str_eq(&xml, want, "not equal");
}
