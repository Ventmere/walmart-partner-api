use std::io::Read;
use error::*;
use response::JsonMaybe;
mod types;
use serde_urlencoded;

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

impl Client {
  pub fn get_all_feed_statuses(&self, query: &GetAllFeedStatusesQuery) -> Result<FeedStatuses> {
    let qs = serde_urlencoded::to_string(query)?;
    self.request_json(Method::Get, "/v3/feeds", qs)?
      .send()?
      .json_maybe::<FeedStatuses>()
      .map_err(Into::into)
  }

  pub fn get_feed_and_item_status(&self, feed_id: &str, query: &GetFeedAndItemStatusQuery) -> Result<PartnerFeedResponse> {
    let path = format!("/v3/feeds/{}", feed_id);
    self.request_json(Method::Get, &path, serde_urlencoded::to_string(query)?)?
      .send()?
      .json_maybe::<PartnerFeedResponse>()
      .map_err(Into::into)
  }

  pub fn bulk_upload<R: Read + Send + 'static>(&self, feed_type: &str, feed: R) -> Result<FeedAck> {
    use multipart::client::lazy::Multipart;
    use reqwest::header::ContentType;

    let mut multipart = Multipart::new();
    multipart.add_stream::<_, _, &str>("file", feed, None, None);
    let mut multipart_prepared = multipart.prepare().unwrap();
    let mut multipart_buffer: Vec<u8> = vec![];
    multipart_prepared
      .read_to_end(&mut multipart_buffer)
      .unwrap();

    self.request_json(Method::Post, "/v2/feeds", vec![
      ("feedType", feed_type)
    ])?
      .body(multipart_buffer)
      .header(ContentType("multipart/form-data".parse().unwrap()))
      .send()?
      .json_maybe::<FeedAck>()
      .map_err(Into::into)
  }
}