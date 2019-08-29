use crate::response::JsonMaybe;
use crate::result::*;
use std::io::Read;
mod types;
use serde_urlencoded;

pub use self::types::*;
use crate::client::{Client, Method};

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
  pub fn get_all_feed_statuses(
    &self,
    query: &GetAllFeedStatusesQuery,
  ) -> WalmartResult<FeedStatuses> {
    let qs = serde_urlencoded::to_string(query)?;
    self
      .send(self.request_json(Method::GET, "/v3/feeds", qs)?)?
      .json_maybe::<FeedStatuses>()
      .map_err(Into::into)
  }

  pub fn get_feed_and_item_status(
    &self,
    feed_id: &str,
    query: &GetFeedAndItemStatusQuery,
  ) -> WalmartResult<PartnerFeedResponse> {
    let path = format!("/v3/feeds/{}", feed_id);
    self
      .send(self.request_json(Method::GET, &path, serde_urlencoded::to_string(query)?)?)?
      .json_maybe::<PartnerFeedResponse>()
      .map_err(Into::into)
  }

  pub fn bulk_upload<R: Read + Send + 'static>(
    &self,
    feed_type: &str,
    feed: R,
  ) -> WalmartResult<FeedAck> {
    use multipart::client::lazy::Multipart;
    use reqwest::header::{HeaderValue, CONTENT_TYPE};

    let mut multipart = Multipart::new();
    multipart.add_stream::<_, _, &str>("file", feed, None, None);
    let mut multipart_prepared = multipart.prepare().unwrap();
    let mut multipart_buffer: Vec<u8> = vec![];
    multipart_prepared
      .read_to_end(&mut multipart_buffer)
      .unwrap();
    self
      .send(
        self
          .request_json(Method::POST, "/v2/feeds", vec![("feedType", feed_type)])?
          .body(multipart_buffer)
          .header(
            CONTENT_TYPE,
            HeaderValue::from_static("multipart/form-data"),
          ),
      )?
      .json_maybe::<FeedAck>()
      .map_err(Into::into)
  }
}
