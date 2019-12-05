use crate::utils::deserialize_timestamp;
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
  #[serde(deserialize_with = "deserialize_timestamp")]
  pub feedDate: DateTime<Utc>,
  #[serde(deserialize_with = "deserialize_timestamp")]
  pub modifiedDtm: DateTime<Utc>,
  pub fileName: Option<String>,
  pub itemDataErrorCount: i32,
  pub itemSystemErrorCount: i32,
  pub itemTimeoutErrorCount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedStatusesResults {
  pub feed: Vec<FeedStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FeedStatuses {
  pub totalResults: i32,
  pub offset: i32,
  pub limit: i32,
  pub results: FeedStatusesResults,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ItemIngestionStatus {
  pub martId: i32,
  pub sku: String,
  pub wpid: String,
  pub ingestionStatus: String,
  pub ingestionErrors: IngestionErrors,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct IngestionErrors {
  pub ingestionError: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ItemDetails {
  pub itemIngestionStatus: Vec<ItemIngestionStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PartnerFeedResponse {
  pub feedId: String,
  pub feedStatus: String,
  pub ingestionErrors: Option<IngestionErrors>,
  pub itemsReceived: i32,
  pub itemsSucceeded: i32,
  pub itemsFailed: i32,
  pub itemsProcessing: i32,
  pub offset: i32,
  pub limit: i32,
  pub itemDetails: ItemDetails,
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::from_str;

  #[test]
  fn deserialize_partner_feed_response() {
    from_str::<PartnerFeedResponse>(
      r##"{
      "feedId": "117E39F0B7654B08A059457FB6E803FF@AQYBAAA",
      "feedStatus": "PROCESSED",
      "shipNode": null,
      "ingestionErrors": {
        "ingestionError": null
      },
      "itemsReceived": 1,
      "itemsSucceeded": 0,
      "itemsFailed": 1,
      "itemsProcessing": 0,
      "offset": 0,
      "limit": 50,
      "itemDetails": {
        "itemIngestionStatus": []
      }
    }"##,
    )
    .unwrap();
  }
}
