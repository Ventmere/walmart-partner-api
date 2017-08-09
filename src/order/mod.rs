use error::*;
use chrono::{DateTime, Utc};
use serde_qs;

mod types;

pub use self::types::*;
use response::{ListResponse, parse_list_elements_json, parse_object_json};
use client::{Method, Client};

/// Query parameters for `get_all_released_orders`
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct ReleasedQueryParams {
  pub limit: Option<i32>,
  pub createdStartDate: DateTime<Utc>,
  pub nextCursor: Option<String>,
}

/// Query parameters for `get_all_orders`
#[derive(Debug, Serialize)]
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
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct ShipParams {
  pub lineNumber: String,
  pub shipDateTime: DateTime<Utc>,
  pub carrierName: String,
  pub methodCode: String,
  pub trackingNumber: String,
  pub trackingURL: String,
}

pub type OrderList = ListResponse<Order>;

pub trait OrderApi {
  fn get_all_released_orders(&self, params: &ReleasedQueryParams) -> Result<OrderList>;
  fn get_all_orders(&self, params: &QueryParams) -> Result<OrderList>;
  fn get_order(&self, purchase_order_id: &str) -> Result<Order>;
  fn ack_order(&self, purchase_order_id: &str) -> Result<Order>;
  // fn cancel_order_line(...) -> Result<Order>;
  // fn refund_order_line(...) -> Result<Order>;
  fn ship_order_line(&self, purchase_order_id: &str, params: &ShipParams) -> Result<Order>;
}

impl OrderApi for Client {
  fn get_all_released_orders(&self, params: &ReleasedQueryParams) -> Result<OrderList> {
    let mut res = self.request_json(Method::Get, "/v3/orders/released", serde_qs::to_string(params)?)?
      .send()?;
    parse_list_elements_json(res.status(), &mut res, "order")
      .map_err(Into::into)
  }

  fn get_all_orders(&self, params: &QueryParams) -> Result<OrderList> {
    let mut res = self.request_json(Method::Get, "/v3/orders", serde_qs::to_string(params)?)?
      .send()?;
    parse_list_elements_json(res.status(), &mut res, "order")
      .map_err(Into::into)
  }

  fn get_order(&self, purchase_order_id: &str) -> Result<Order> {
    let path = format!("/v3/orders/{}", purchase_order_id);
    let mut res = self.request_json(Method::Get, &path, ())?
      .send()?;
    parse_object_json(res.status(), &mut res, "order")
      .map_err(Into::into)
  }

  fn ack_order(&self, purchase_order_id: &str) -> Result<Order> {
    let path = format!("/v3/orders/{}/acknowledge", purchase_order_id);
    let mut res = self.request_json(Method::Post, &path, ())?
      .send()?;
    parse_object_json(res.status(), &mut res, "order")
      .map_err(Into::into)
  }

  fn ship_order_line(&self, purchase_order_id: &str, params: &ShipParams) -> Result<Order> {
    use serde_json::Value;
    let timestamp: i64 = params.shipDateTime.timestamp() * 1000 + params.shipDateTime.timestamp_subsec_millis() as i64;
    let body = json!({
      "orderShipment": {
        "orderLines": {
          "orderLine": [
            {
              "lineNumber": params.lineNumber,
              "orderLineStatuses": {
                "orderLineStatus": [
                  {
                    "status": "Shipped",
                    "statusQuantity": {
                      "unitOfMeasurement": "EA",
                      "amount": "1"
                    },
                    "trackingInfo": {
                      "shipDateTime": timestamp,
                      "carrierName": {
                        "otherCarrier": Value::Null,
                        "carrier": params.carrierName
                      },
                      "methodCode": params.methodCode,
                      "trackingNumber": params.trackingNumber,
                      "trackingURL": params.trackingURL
                    }
                  }
                ]
              }
            }
          ]
        }
      }
    });
    let path = format!("/v3/orders/{}/shipping", purchase_order_id);
    let mut res = self.request_json(Method::Post, &path, vec![
      ("purchaseOrderId", purchase_order_id)
    ])?
      .json(&body)?
      .send()?;
    parse_object_json(res.status(), &mut res, "order")
      .map_err(Into::into)
  }
}