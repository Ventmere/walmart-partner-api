use crate::result::*;
mod types;
use crate::client::{Client, Method};
use crate::xml::Xml;

pub use self::types::*;

/// Query parameters for `get_all_items`
#[derive(Debug, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct GetAllItemsQueryParams {
  pub nextCursor: String,
  pub sku: Option<String>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
}

impl Default for GetAllItemsQueryParams {
  fn default() -> Self {
    GetAllItemsQueryParams {
      nextCursor: "*".to_string(),
      sku: None,
      limit: None,
      offset: None,
    }
  }
}

impl Client {
  pub fn get_all_items(
    &self,
    params: &GetAllItemsQueryParams,
  ) -> WalmartResult<(Xml<GetAllItems>, Option<GetAllItemsQueryParams>)> {
    let qs = serde_urlencoded::to_string(params)?;
    let mut res = self.send(self.request_xml(Method::GET, "/v3/items", qs)?)?;

    let xml = Xml::<GetAllItems>::from_res(&mut res)?;
    let next_params = xml.get_next_query_params(params, self.get_marketplace());
    Ok((xml, next_params))
  }
}
