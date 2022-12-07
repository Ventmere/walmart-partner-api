use chrono::{DateTime, Utc};

use crate::shared::error::ResponseError;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAllFeedStatusesQuery {
  pub feed_id: Option<String>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFeedAndItemStatusQuery {
  pub include_details: Option<bool>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFeedStatuses {
  #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
  pub errors: Option<Vec<ResponseError>>,
  /// Total number of feeds returned
  #[serde(rename = "totalResults", skip_serializing_if = "Option::is_none")]
  pub total_results: Option<i64>,
  /// The object response to the starting number, where 0 is the first available
  #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
  pub offset: Option<i64>,
  /// The number of items to be returned
  #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
  pub limit: Option<i32>,
  #[serde(rename = "results")]
  #[serde(default)]
  pub results: FeedResults,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedResults {
  /// The feed status results
  #[serde(rename = "feed")]
  #[serde(default)]
  pub feed: Vec<FeedResult>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedResult {
  /// A unique ID used for tracking the Feed File
  #[serde(rename = "feedId", skip_serializing_if = "Option::is_none")]
  pub feed_id: Option<String>,
  /// The source of the feed
  #[serde(rename = "feedSource", skip_serializing_if = "Option::is_none")]
  pub feed_source: Option<String>,
  /// The feed type
  #[serde(rename = "feedType", skip_serializing_if = "Option::is_none")]
  pub feed_type: Option<String>,
  /// The seller ID
  #[serde(rename = "partnerId", skip_serializing_if = "Option::is_none")]
  pub partner_id: Option<String>,
  /// The number of items received
  #[serde(rename = "itemsReceived", skip_serializing_if = "Option::is_none")]
  pub items_received: Option<i64>,
  /// The number of items in the feed that have successfully processed
  #[serde(rename = "itemsSucceeded", skip_serializing_if = "Option::is_none")]
  pub items_succeeded: Option<i64>,
  /// The number of items in the feed that failed due to a data or system error
  #[serde(rename = "itemsFailed", skip_serializing_if = "Option::is_none")]
  pub items_failed: Option<i64>,
  /// The number of items in the feed that are still in progress
  #[serde(rename = "itemsProcessing", skip_serializing_if = "Option::is_none")]
  pub items_processing: Option<i64>,
  /// Can be one of the following: RECEIVED, INPROGRESS, PROCESSED, or ERROR. For details, see the definitions listed under 'Feed Statuses' at the beginning of this section.
  #[serde(rename = "feedStatus", skip_serializing_if = "Option::is_none")]
  pub feed_status: Option<String>,
  /// The date and time the feed was submitted. Format: yyyymmddThh:mm:ss.xxxz
  #[serde(rename = "feedDate", skip_serializing_if = "Option::is_none")]
  pub feed_date: Option<DateTime<Utc>>,
  /// The batch ID for the feed, if provided
  #[serde(rename = "batchId", skip_serializing_if = "Option::is_none")]
  pub batch_id: Option<String>,
  /// The most recent time the feed was modified. Format: yyyymmddThh:mm:ss.xxxz
  #[serde(rename = "modifiedDtm", skip_serializing_if = "Option::is_none")]
  pub modified_dtm: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFeedItemStatus {
  #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
  pub errors: Option<Vec<ResponseError>>,
  /// A unique ID used for tracking the Feed File
  #[serde(rename = "feedId", skip_serializing_if = "Option::is_none")]
  pub feed_id: Option<String>,
  /// Can be one of the following: RECEIVED, INPROGRESS, PROCESSED, or ERROR
  #[serde(rename = "feedStatus", skip_serializing_if = "Option::is_none")]
  pub feed_status: Option<String>,
  /// The number of items received in the feed
  #[serde(rename = "itemsReceived", skip_serializing_if = "Option::is_none")]
  pub items_received: Option<i32>,
  /// The number of items in the feed that processed successfully
  #[serde(rename = "itemsSucceeded", skip_serializing_if = "Option::is_none")]
  pub items_succeeded: Option<i32>,
  /// The number of items in the feed that failed due to a data or system error
  #[serde(rename = "itemsFailed", skip_serializing_if = "Option::is_none")]
  pub items_failed: Option<i32>,
  /// The number of items in the feed that are still processing
  #[serde(rename = "itemsProcessing", skip_serializing_if = "Option::is_none")]
  pub items_processing: Option<i32>,
  /// The object response to the starting number, where 0 is the first entity available for request
  #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
  pub offset: Option<i32>,
  /// The number of items returned. Cannot be greater than 1000.
  #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
  pub limit: Option<i32>,
  #[serde(rename = "itemDetails")]
  #[serde(default)]
  pub item_details: FeedItemDetails,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedItemDetails {
  /// The ingestion status of an individual item
  #[serde(rename = "itemIngestionStatus")]
  #[serde(default)]
  pub item_ingestion_status: Vec<FeedItemIngestionStatus>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedItemIngestionStatus {
  /// Mart ID that a user or seller uses for a marketplace
  #[serde(rename = "martId", skip_serializing_if = "Option::is_none")]
  pub mart_id: Option<i32>,
  /// An arbitrary alphanumeric unique ID, seller-specified, identifying each item.
  #[serde(rename = "sku", skip_serializing_if = "Option::is_none")]
  pub sku: Option<String>,
  /// An alphanumeric product ID, generated by Walmart
  #[serde(rename = "wpid", skip_serializing_if = "Option::is_none")]
  pub wpid: Option<String>,
  /// index of items in the feed
  #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
  pub index: Option<i32>,
  /// Can be one of the following: DATA_ERROR, SYSTEM_ERROR, TIMEOUT_ERROR, or INPROGRESS
  #[serde(rename = "ingestionStatus")]
  pub ingestion_status: String,
  #[serde(rename = "ingestionErrors", skip_serializing_if = "Option::is_none")]
  pub ingestion_errors: Option<FeedItemIngestionErrors>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedItemIngestionErrors {
  #[serde(rename = "ingestionError", skip_serializing_if = "Option::is_none")]
  pub ingestion_error: Option<Vec<FeedItemIngestionError>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedItemIngestionError {
  /// Error Type
  #[serde(rename = "type")]
  pub r#type: String,
  /// Error code
  #[serde(rename = "code")]
  pub code: String,
  /// Error description
  #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}
