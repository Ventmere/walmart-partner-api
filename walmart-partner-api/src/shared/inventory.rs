use xml_builder::XMLElement;

use crate::WalmartResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryQuantity {
  /// The unit of measurement. Example: 'EACH'
  #[serde(rename = "unit")]
  pub unit: String,
  /// The number available in the inventory
  #[serde(rename = "amount")]
  pub amount: i32,
}

impl crate::XmlSer for InventoryQuantity {
  fn to_xml(&self) -> WalmartResult<XMLElement> {
    let mut root = XMLElement::new("quantity");

    let mut unit = XMLElement::new("unit");
    unit.add_text(self.unit.clone())?;
    root.add_child(unit)?;

    let mut amount = XMLElement::new("amount");
    amount.add_text(self.amount.to_string())?;
    root.add_child(amount)?;

    Ok(root)
  }
}
