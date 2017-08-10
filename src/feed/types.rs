use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FeedAck {
  pub feedId: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FeedStatus {
  pub feedId: String,
  pub feedType: String,
  pub partnerId: String,
  pub itemsReceived: i32,
  pub itemsSucceeded: i32,
  pub itemsFailed: i32,
  pub itemsProcessing: i32,
  pub feedStatus: String,
  pub feedDate: DateTime<Utc>,
  pub modifiedDtm: DateTime<Utc>,
  pub fileName: String,
  pub itemDataErrorCount: i32,
  pub itemSystemErrorCount: i32,
  pub itemTimeoutErrorCount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FeedStatuses {
  pub totalResults: i32,
  pub offset: i32,
  pub limit: i32,
  pub results: Vec<FeedStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ItemIngestionStatus {
  pub martId: i32,
  pub sku: String,
  pub wpid: String,
  pub ingestionStatus: String,
  pub ingestionErrors: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PartnerFeedResponse {
  pub feedId: String,
  pub feedStatus: String,
  pub ingestionErrors: Vec<Value>,
  pub itemsReceived: i32,
  pub itemsSucceeded: i32,
  pub itemsFailed: i32,
  pub itemsProcessing: i32,
  pub offset: i32,
  pub limit: i32,
  pub itemDetails: Vec<ItemIngestionStatus>,
}