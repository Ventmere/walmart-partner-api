use std::io::Read;
use error::*;
mod types;

pub use self::types::*;
use client::{Method, Client};

#[derive(Debug, Serialize, Default)]
#[allow(non_snake_case)]
pub struct GetAllFeedStatusesQuery<'a> {
  pub feedId: Option<&'a str>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
}

#[derive(Debug, Serialize, Default)]
#[allow(non_snake_case)]
pub struct GetFeedAndItemStatusQuery {
  pub includeDetails: Option<bool>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
}

pub trait FeedApi {
  fn get_all_feed_statuses(&self, query: &GetAllFeedStatusesQuery) -> Result<FeedStatuses>;
  fn get_feed_and_item_status(&self, feed_id: &str, query: &GetFeedAndItemStatusQuery) -> Result<PartnerFeedResponse>;
  fn bulk_upload<R: Read>(&self, feed_type: &str, feed: R) -> Result<FeedAck>;
}