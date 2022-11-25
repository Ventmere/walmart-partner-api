use crate::result::*;
use crate::utils::deserialize_timestamp;
use crate::xml::{Element, FromXmlElement};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFeedAndItemStatusQuery {
  pub include_details: Option<bool>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAllFeedStatusesQuery {
  pub feed_id: Option<String>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedAcknowledgement {
  pub feed_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedStatus {
  pub feed_id: String,
  pub feed_type: String,
  pub partner_id: String,
  pub items_received: i32,
  pub items_succeeded: i32,
  pub items_failed: i32,
  pub items_processing: i32,
  pub feed_status: String,
  pub feed_date: DateTime<Utc>,
  pub modified_dtm: DateTime<Utc>,
  pub file_mame: Option<String>,
  pub item_data_error_count: i32,
  pub item_system_error_count: i32,
  pub item_timeout_error_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedStatusesResults {
  pub feed: Vec<FeedStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedStatuses {
  pub total_results: i32,
  pub offset: i32,
  pub limit: i32,
  pub results: FeedStatusesResults,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemIngestionStatus {
  pub mart_id: i32,
  pub sku: String,
  pub wpid: String,
  pub ingestion_status: String,
  pub ingestion_errors: Option<IngestionErrors>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestionErrors {
  pub ingestion_error: Vec<IngestionError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngestionError {
  pub type_: String,
  pub code: String,
  pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetails {
  pub item_ingestion_status: Vec<ItemIngestionStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartnerFeedResponse {
  pub feed_id: String,
  pub feed_status: String,
  pub ingestion_errors: Option<IngestionErrors>,
  pub items_received: i32,
  pub items_succeeded: i32,
  pub items_failed: i32,
  pub items_processing: i32,
  pub offset: i32,
  pub limit: i32,
  pub item_details: ItemDetails,
}
