use reqwest::Response;
use result::*;
use xmltree::Element;

pub trait FromXmlElement: Sized {
  fn from_xml_element(elem: Element) -> WalmartResult<Self>;
}

pub struct Xml<T> {
  inner: T,
  text: String,
}

impl<T> ::std::ops::Deref for Xml<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<T> Xml<T>
where
  T: FromXmlElement,
{
  pub fn from_res(res: &mut Response) -> WalmartResult<Self> {
    use std::io::Cursor;

    let status = res.status();
    let text = res.text().map_err(|err| ApiResponseError {
      message: format!("get response text: {}", err.to_string()),
      status: status.clone(),
      body: "".to_string(),
    })?;

    let elem = Element::parse(Cursor::new(text.as_bytes())).map_err(|err| ApiResponseError {
      message: format!("parse response xml: {}", err.to_string()),
      status: status.clone(),
      body: text.clone(),
    })?;

    let inner = T::from_xml_element(elem)?;

    Ok(Xml { inner, text })
  }

  pub fn text(&self) -> &str {
    self.text.as_ref()
  }

  pub fn into_inner(self) -> T {
    self.inner
  }
}

pub trait GetChildText {
  fn get_child_text(&self, name: &str) -> Option<String>;
  fn get_child_text_or_default(&self, name: &str) -> String {
    self.get_child_text(name).unwrap_or_default()
  }
}

impl GetChildText for Element {
  fn get_child_text(&self, name: &str) -> Option<String> {
    self.get_child(name).and_then(|c| c.text.clone())
  }
}
