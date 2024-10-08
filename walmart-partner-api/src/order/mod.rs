use crate::result::*;
use chrono::{DateTime, Utc};
use serde_json::Value;
use serde_urlencoded;

mod types;

pub use self::types::*;
use crate::client::{Client, Method};
use crate::response::{parse_list_elements_json, parse_object_json, ListResponse};

/// Query parameters for `get_all_released_orders`

#[derive(Debug, Serialize, Default)]
#[allow(non_snake_case)]
pub struct ReleasedQueryParams {
  pub limit: Option<i32>,
  pub createdStartDate: Option<DateTime<Utc>>,
  pub nextCursor: Option<String>,
}

/// Query parameters for `get_all_orders`
#[derive(Debug, Serialize, Default)]
#[allow(non_snake_case)]
pub struct QueryParams {
  pub sku: Option<String>,
  pub customerOrderId: Option<String>,
  pub purchaseOrderId: Option<String>,
  pub status: Option<String>,
  pub createdStartDate: Option<DateTime<Utc>>,
  pub createdEndDate: Option<DateTime<Utc>>,
  pub fromExpectedShipDate: Option<DateTime<Utc>>,
  pub toExpectedShipDate: Option<DateTime<Utc>>,
  pub limit: Option<i32>,
  pub nextCursor: Option<String>,
  pub shipNodeType: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct WFSQueryParams {
  pub customerOrderId: Option<String>,
  pub createdStartDate: Option<DateTime<Utc>>,
  pub createdEndDate: Option<DateTime<Utc>>,
  pub status: Option<String>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
}

#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct ShipParams {
  pub lineNumber: String,
  pub shipDateTime: DateTime<Utc>,
  pub carrierName: Option<String>,
  pub methodCode: String,
  pub trackingNumber: String,
  pub trackingURL: String,
  pub otherCarrier: Option<String>,
  pub unitOfMeasurement: Option<String>,
  pub amount: Option<String>,
  pub shipFromCountry: String,
}

impl ShipParams {
  pub fn to_value(&self) -> Value {
    let timestamp: i64 =
      self.shipDateTime.timestamp() * 1000 + self.shipDateTime.timestamp_subsec_millis() as i64;
    json!({
      "lineNumber": self.lineNumber,
      "shipFromCountry": self.shipFromCountry,
      "orderLineStatuses": {
        "orderLineStatus": [
          {
            "status": "Shipped",
            "statusQuantity": {
              "unitOfMeasurement": self.unitOfMeasurement.clone().unwrap_or_else(|| "EACH".to_owned()),
              "amount": self.amount.clone().unwrap_or_else(|| "1".to_owned()),
            },
            "trackingInfo": {
              "shipDateTime": timestamp,
              "carrierName": {
                "otherCarrier": self.otherCarrier,
                "carrier": self.carrierName,
              },
              "methodCode": self.methodCode,
              "trackingNumber": self.trackingNumber,
              "trackingURL": self.trackingURL
            }
          }
        ]
      }
    })
  }
}

pub type OrderList = ListResponse<Order>;
pub type OrderWFSList = ListResponse<OrderWFS>;

impl Client {
  pub fn get_all_released_orders(&self, params: &ReleasedQueryParams) -> WalmartResult<OrderList> {
    let qs = serde_urlencoded::to_string(params)?;
    let mut res = self.send(self.request_json(Method::GET, "/v3/orders/released", qs)?)?;
    parse_list_elements_json(res.status(), &mut res, "order").map_err(Into::into)
  }

  pub fn get_all_orders(&self, params: &QueryParams) -> WalmartResult<OrderList> {
    let mut res = self.send(self.request_json(
      Method::GET,
      "/v3/orders",
      serde_urlencoded::to_string(params)?,
    )?)?;
    parse_list_elements_json(res.status(), &mut res, "order").map_err(Into::into)
  }

  /// Get all WFS orders (Only Canada)
  pub fn get_all_wfs_orders(&self, params: &WFSQueryParams) -> WalmartResult<OrderWFSList> {
    let mut res = self.send(self.request_json(
      Method::GET,
      "/v3/orders/wfs",
      serde_urlencoded::to_string(params)?,
    )?)?;
    parse_list_elements_json(res.status(), &mut res, "order").map_err(Into::into)
  }

  pub fn get_all_orders_by_next_cursor(&self, next_cursor: &str) -> WalmartResult<OrderList> {
    use url::form_urlencoded;
    let mut res = self.send(
      self.request_json(
        Method::GET,
        "/v3/orders",
        form_urlencoded::parse((&next_cursor).as_bytes())
          .into_owned()
          .collect::<Vec<_>>(),
      )?,
    )?;
    parse_list_elements_json(res.status(), &mut res, "order").map_err(Into::into)
  }

  pub fn get_order(&self, purchase_order_id: &str) -> WalmartResult<Order> {
    let path = format!("/v3/orders/{}", purchase_order_id);
    let mut res = self.send(self.request_json(Method::GET, &path, ())?)?;
    parse_object_json(res.status(), &mut res, "order").map_err(Into::into)
  }

  pub fn ack_order(&self, purchase_order_id: &str) -> WalmartResult<Order> {
    let path = format!("/v3/orders/{}/acknowledge", purchase_order_id);
    let mut res = self.send(
      self
        .request_json(Method::POST, &path, ())?
        .json(&Vec::<i32>::new()),
    )?;
    parse_object_json(res.status(), &mut res, "order").map_err(Into::into)
  }

  pub fn ship_order_line(
    &self,
    purchase_order_id: &str,
    line: &ShipParams,
  ) -> WalmartResult<Order> {
    self.ship_order(purchase_order_id, &[line.clone()])
  }

  pub fn ship_order(&self, purchase_order_id: &str, lines: &[ShipParams]) -> WalmartResult<Order> {
    let line_values: Vec<_> = lines.into_iter().map(ShipParams::to_value).collect();
    let body = json!({
      "orderShipment": {
        "orderLines": {
          "orderLine": line_values,
        }
      }
    });
    let path = format!("/v3/orders/{}/shipping", purchase_order_id);
    let mut res = self.send(
      self
        .request_json(
          Method::POST,
          &path,
          vec![("purchaseOrderId", purchase_order_id)],
        )?
        .json(&body),
    )?;
    parse_object_json(res.status(), &mut res, "order").map_err(Into::into)
  }
}
