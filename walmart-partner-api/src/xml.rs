use crate::WalmartResult;

pub trait XmlSer {
  fn to_xml(&self) -> WalmartResult<xml_builder::XMLElement>;

  fn get_element_with_text<T: ToString>(
    &self,
    name: &str,
    text: T,
  ) -> WalmartResult<xml_builder::XMLElement> {
    let mut elem = xml_builder::XMLElement::new(name);
    elem.add_text(text.to_string())?;
    Ok(elem)
  }

  fn to_string(&self) -> WalmartResult<String> {
    use xml_builder::{XMLBuilder, XMLVersion};
    let mut xml = XMLBuilder::new()
      .version(XMLVersion::XML1_0)
      .encoding("UTF-8".into())
      .build();
    xml.set_root_element(self.to_xml()?);
    let mut writer = Vec::<u8>::new();
    xml.generate(&mut writer)?;
    Ok(String::from_utf8_lossy(&writer).to_string())
  }
}

pub fn get_element_with_text<T: ToString>(
  name: &str,
  text: T,
) -> WalmartResult<xml_builder::XMLElement> {
  let mut elem = xml_builder::XMLElement::new(name);
  elem.add_text(text.to_string())?;
  Ok(elem)
}
