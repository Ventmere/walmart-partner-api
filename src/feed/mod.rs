use std::io::Read;
use error::*;
use response::JsonMaybe;
mod types;
use serde_qs;

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
  fn bulk_upload<R: Read + Send + 'static>(&self, feed_type: &str, feed: R) -> Result<FeedAck>;
}

impl FeedApi for Client {
  fn get_all_feed_statuses(&self, query: &GetAllFeedStatusesQuery) -> Result<FeedStatuses> {
    self.request_json(Method::Get, "/v3/feeds", serde_qs::to_string(query)?)?
      .send()?
      .json_maybe::<FeedStatuses>()
      .map_err(Into::into)
  }

  fn get_feed_and_item_status(&self, feed_id: &str, query: &GetFeedAndItemStatusQuery) -> Result<PartnerFeedResponse> {
    let path = format!("/v3/feeds/{}", feed_id);
    self.request_json(Method::Get, &path, serde_qs::to_string(query)?)?
      .send()?
      .json_maybe::<PartnerFeedResponse>()
      .map_err(Into::into)
  }

  fn bulk_upload<R: Read + Send + 'static>(&self, feed_type: &str, feed: R) -> Result<FeedAck> {
    use multipart::client::lazy::Multipart;

    let mut multipart = Multipart::new();
    multipart.add_stream("file", feed, Some("feed.xlsx"), None);
    let mut multipart_prepared = multipart.prepare().unwrap();
    let mut multipart_buffer: Vec<u8> = vec![];
    multipart_prepared
      .read_to_end(&mut multipart_buffer)
      .unwrap();

    self.request_json(Method::Post, "/v2/feeds", vec![
      ("feedType", feed_type)
    ])?
      .body(multipart_buffer)
      .send()?
      .json_maybe::<FeedAck>()
      .map_err(Into::into)
  }
}