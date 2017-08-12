use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{self, Value};
use reqwest::{StatusCode, Response};
use error::ApiResponseError;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ListMeta {
  pub totalCount: i32,
  pub limit: i32,
  pub nextCursor: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListResponse<T: Serialize> {
  pub meta: Option<ListMeta>,
  pub elements: Vec<T>,
}

pub type Result<T> = ::std::result::Result<T, ApiResponseError>;

pub trait JsonMaybe {
  fn json_maybe<T: DeserializeOwned>(&mut self) -> Result<T>;
}

impl JsonMaybe for Response {
  fn json_maybe<T: DeserializeOwned>(&mut self) -> Result<T> {
    let status = self.status();

    let mut body = String::new();
    match self.read_to_string(&mut body) {
      Err(err) => {
        return Err(ApiResponseError {
          status: status.clone(),
          message: format!("read response: {}", err),
          body: "".to_owned(),
        });
      },
      _ => {},
    }

    if !status.is_success() {
      return Err(ApiResponseError {
        message: format!("status not ok: {}", status),
        status: status.clone(),
        body: body,
      });
    }

    match serde_json::from_str::<T>(&body) {
      Ok(v) => {
        return Ok(v);
      },
      Err(err) => {
        return Err(ApiResponseError {
          message: format!("deserialize body: {}", err),
          status: status.clone(),
          body: body,
        });
      }
    }
  }
}

/// Get `meta` and `elements` from a JSON API response
pub fn parse_list_elements_json<T, R>(status: StatusCode, reader: &mut R, key: &str) -> Result<ListResponse<T>> 
  where T: Serialize + DeserializeOwned, R: Read
{
  use std::collections::BTreeMap;  

  #[derive(Debug, Deserialize)]
  pub struct Inner {
    pub meta: ListMeta,
    pub elements: BTreeMap<String, Value>,
  }

  #[derive(Debug, Deserialize)]
  pub struct Response {
    pub list: Inner,
  }

  if status == StatusCode::NotFound {
    return Ok(ListResponse {
      meta: None,
      elements: vec![],
    })
  }

  let mut body = String::new();
  match reader.read_to_string(&mut body) {
    _ => {},
  }

  if !status.is_success() {
    return Err(ApiResponseError {
      message: status.to_string(),
      status: status.clone(),
      body: body,
    });
  }

  match serde_json::from_str::<Response>(&body) {
    Ok(mut res) => {
      let value = match res.list.elements.remove(key) {
        Some(value) => value,
        None => {
          return Err(ApiResponseError {
            message: format!("key '{}' was not found in resposne", key),
            status: status.clone(),
            body: body,
          });
        }
      };

      match serde_json::from_value::<Vec<T>>(value) {
        Ok(elements) => {
          return Ok(ListResponse {
            meta: res.list.meta.into(),
            elements: elements,
          });
        },
        Err(err) => {
          return Err(ApiResponseError {
            message: format!("deserialize json response elements: {}", err.to_string()),
            status: status.clone(),
            body: body,
          });
        }
      }
    },
    Err(err) => {
      return Err(ApiResponseError {
        message: format!("deserialize json response: {}", err.to_string()),
        status: status.clone(),
        body: body,
      });
    }
  }
}

/// Get single object from a JSON API response
pub fn parse_object_json<T, R>(status: StatusCode, reader: &mut R, key: &str) -> Result<T> 
  where T: Serialize + DeserializeOwned, R: Read
{
  use std::collections::BTreeMap;

  let mut body = String::new();
  match reader.read_to_string(&mut body) {
    _ => {},
  }

  if !status.is_success() {
    return Err(ApiResponseError {
      message: status.to_string(),
      status: status.clone(),
      body: body,
    });
  }

  match serde_json::from_str::<BTreeMap<String, T>>(&body) {
    Ok(mut obj) => {
      match obj.remove(key) {
        Some(value) => {
          return Ok(value);
        },
        None => {
          return Err(ApiResponseError {
            message: format!("key '{}' was not found in resposne", key),
            status: status.clone(),
            body: body,
          });
        }
      }
    },
    Err(err) => {
      return Err(ApiResponseError {
        message: format!("deserialize json response: {}", err.to_string()),
        status: status.clone(),
        body: body,
      });
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::io::Cursor;

  #[test]
  fn test_parse_list_elements_json() {
    use order::Order;
    let mut r = Cursor::new(include_str!("./order/test_order_list_res.json").to_string());
    let res = parse_list_elements_json::<Order, _>(StatusCode::Ok, &mut r, "order").unwrap();
    let meta = res.meta.unwrap();
    assert_eq!(meta.totalCount, 66);
    assert_eq!(meta.limit, 10);
    assert_eq!(res.elements.len(), 2);
  }

  #[test]
  fn test_parse_object_json() {
    use order::Order;
    let mut r = Cursor::new(include_str!("./order/test_order.json").to_string());
    let res = parse_object_json::<Order, _>(StatusCode::Ok, &mut r, "order").unwrap();
    assert_eq!(res.shippingInfo.estimatedDeliveryDate, 1485586800000);
  }
}