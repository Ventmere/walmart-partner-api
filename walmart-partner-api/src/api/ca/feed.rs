use crate::api::ca::Client;
use crate::client::Method;
pub use crate::feed::*;
use crate::result::WalmartResult;
use std::io::Read;

impl Client {
  pub async fn get_all_feed_statuses(
    &self,
    query: &GetAllFeedStatusesQuery,
  ) -> WalmartResult<FeedStatuses> {
    let qs = serde_urlencoded::to_string(&query)?;
    let req = self.req_xml(Method::GET, "/v3/feeds", qs)?;
    self.send(req).await?.res_xml().await
  }

  pub async fn get_feed_and_item_status(
    &self,
    feed_id: &str,
    query: &GetFeedAndItemStatusQuery,
  ) -> WalmartResult<PartnerFeedResponse> {
    let qs = serde_urlencoded::to_string(&query)?;
    let path = format!("/v3/feeds/{}", feed_id);
    let req = self.req_xml(Method::GET, &path, qs)?;
    self.send(req).await?.res_xml().await
  }
}
