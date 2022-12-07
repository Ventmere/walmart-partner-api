pub use crate::shared::feed::*;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FeedAck {
  /// A unique ID, returned from the Bulk Upload API, used for tracking the feed file
  #[serde(rename = "feedId", skip_serializing_if = "Option::is_none")]
  pub feed_id: Option<String>,
}
