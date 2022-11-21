use crate::response::JsonMaybe;
use crate::result::*;
use std::io::Read;
mod types;
use serde::{Deserialize, Serialize};
use serde_urlencoded;

pub use self::types::*;
use crate::client::{Client, Method};
use crate::xml::Xml;

#[derive(Debug, Serialize, Default)]
#[allow(non_snake_case)]
pub struct GetFeedAndItemStatusQuery {
  pub includeDetails: Option<bool>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
}
//
// impl Client {
//   pub fn get_all_feed_statuses(
//     &self,
//     query: &GetAllFeedStatusesQuery,
//   ) -> WalmartResult<FeedStatuses> {
//     let qs = serde_urlencoded::to_string(query)?;
//     self
//       .send(self.req_json(Method::GET, "/v3/feeds", qs)?)?
//       .json_maybe::<FeedStatuses>()
//       .map_err(Into::into)
//   }
//
//   pub fn get_feed_and_item_status(
//     &self,
//     feed_id: &str,
//     query: &GetFeedAndItemStatusQuery,
//   ) -> WalmartResult<PartnerFeedResponse> {
//     let path = format!("/v3/feeds/{}", feed_id);
//     self
//       .send(self.req_json(Method::GET, &path, serde_urlencoded::to_string(query)?)?)?
//       .json_maybe::<PartnerFeedResponse>()
//       .map_err(Into::into)
//   }
//
//   pub fn bulk_upload_xml<R: Read + Send + 'static>(
//     &self,
//     feed_type: &str,
//     feed: R,
//   ) -> WalmartResult<FeedAck> {
//     use reqwest::header::{HeaderValue, CONTENT_TYPE};
//     use reqwest::Body;
//     let mut res = self.send(
//       self
//         .req_xml(Method::POST, "/v3/feeds", vec![("feedType", feed_type)])?
//         .body(Body::new(feed))
//         .header(CONTENT_TYPE, HeaderValue::from_static("application/xml")),
//     )?;
//     let xml = Xml::<FeedAck>::from_res(&mut res)?;
//     Ok(xml.into_inner())
//   }
// }
