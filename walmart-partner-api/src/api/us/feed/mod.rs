use std::io::Read;
pub use types::*;

use crate::api::us::Client;
use crate::client::Method;
use crate::result::WalmartResult;

mod types;

impl Client {
  pub async fn get_all_feed_statuses(
    &self,
    query: GetAllFeedStatusesQuery,
  ) -> WalmartResult<GetFeedStatuses> {
    let qs = serde_urlencoded::to_string(&query)?;
    let req = self.req_xml(Method::GET, "/v3/feeds", qs)?;
    self.send(req).await?.res_xml().await
  }

  pub async fn get_feed_and_item_status(
    &self,
    feed_id: impl AsRef<str>,
    query: GetFeedAndItemStatusQuery,
  ) -> WalmartResult<GetFeedItemStatus> {
    let qs = serde_urlencoded::to_string(&query)?;
    let path = format!("/v3/feeds/{}", feed_id.as_ref());
    let req = self.req_xml(Method::GET, &path, qs)?;
    self.send(req).await?.res_xml().await
  }

  pub async fn bulk_upload_xml<R: Read + Send + 'static>(
    &self,
    feed_type: impl AsRef<str>,
    mut feed: R,
  ) -> WalmartResult<FeedAck> {
    let mut buffer = Vec::new();
    feed.read_to_end(&mut buffer)?;
    let part = reqwest::multipart::Part::bytes(buffer);
    let form = reqwest::multipart::Form::new().part("file", part);
    let req = self
      .req_xml(Method::POST, "/v3/feeds", vec![("feedType", feed_type)])?
      .form(form);
    self.send(req).await?.res_xml().await
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use chrono::{DateTime, Utc};
  use mockito::Matcher;

  use super::*;

  #[tokio::test]
  async fn get_all_feed_statuses() {
    let client = crate::test_util::get_client_us();

    let feed_id = "12234EGGT564YTEGFA";
    let limit = 50;
    let _m = mockito::mock("GET", "/v3/feeds")
      .match_query(Matcher::AllOf(vec![
        Matcher::UrlEncoded("feedId".into(), feed_id.into()),
        Matcher::UrlEncoded("limit".into(), limit.to_string()),
      ]))
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(include_str!("./test_get_all_feed_statuses.xml"))
      .create();

    let q = GetAllFeedStatusesQuery {
      feed_id: Some(feed_id.to_string()),
      limit: Some(limit),
      offset: None,
    };
    let got = client.get_all_feed_statuses(q).await.unwrap();
    assert_eq!(got.total_results.unwrap(), 210);
    assert_eq!(got.results.feed.len(), 50);
    let feed = got.results.feed[49].clone();
    assert_eq!(
      feed.feed_id.unwrap(),
      "1F6CF24319FF42FCACFB3D672E7A6F60@AU8BAgA".to_string()
    );
    assert_eq!(feed.feed_type.unwrap(), "item".to_string());
    assert_eq!(feed.partner_id.unwrap(), "100009".to_string());
    assert_eq!(feed.items_received.unwrap(), 1);
    assert_eq!(feed.items_succeeded.unwrap(), 0);
    assert_eq!(feed.items_failed.unwrap(), 1);
    assert_eq!(feed.items_processing.unwrap(), 0);
    assert_eq!(feed.feed_status.unwrap(), "INPROGRESS".to_string());
    assert_eq!(
      feed.feed_date.unwrap(),
      DateTime::<Utc>::from_str("2019-09-12T18:11:34.607Z").unwrap()
    );
  }

  #[tokio::test]
  async fn get_feed_and_item_status() {
    let client = crate::test_util::get_client_us();
    let body = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ns2:PartnerFeedResponse xmlns:ns2="http://walmart.com/">
    <ns2:feedId>F129C19240844B97A3C6AD8F1A2C4997@AU8BAQA</ns2:feedId>
    <ns2:feedStatus>PROCESSED</ns2:feedStatus>
    <ns2:feedSubmissionDate>2019-09-12T17:53:23.059Z</ns2:feedSubmissionDate>
    <ns2:itemsReceived>1</ns2:itemsReceived>
    <ns2:itemsSucceeded>1</ns2:itemsSucceeded>
    <ns2:itemsFailed>0</ns2:itemsFailed>
    <ns2:itemsProcessing>0</ns2:itemsProcessing>
    <ns2:offset>0</ns2:offset>
    <ns2:limit>20</ns2:limit>
    <ns2:itemDetails>
        <ns2:itemIngestionStatus>
            <ns2:martId>0</ns2:martId>
            <ns2:sku>0960B3B82687490FA5E51CB0801478A4@AU8BAgA</ns2:sku>
            <ns2:wpid>71ZLHHMKNS6G</ns2:wpid>
            <ns2:index>0</ns2:index>
            <ns2:itemid>51681142</ns2:itemid>
            <ns2:productIdentifiers>
                <ns2:productIdentifier>
                    <ns2:productIdType>GTIN</ns2:productIdType>
                    <ns2:productId>00363824587165</ns2:productId>
                </ns2:productIdentifier>
                <ns2:productIdentifier>
                    <ns2:productIdType>UPC</ns2:productIdType>
                    <ns2:productId>363824587165</ns2:productId>
                </ns2:productIdentifier>
            </ns2:productIdentifiers>
            <ns2:ingestionStatus>SUCCESS</ns2:ingestionStatus>
        </ns2:itemIngestionStatus>
    </ns2:itemDetails>
</ns2:PartnerFeedResponse>
    ""#;

    let feed_id = "F129C19240844B97A3C6AD8F1A2C4997@AU8BAQA";
    let _m = mockito::mock("GET", format!("/v3/feeds/{}", feed_id).as_str())
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let got = client
      .get_feed_and_item_status(feed_id, Default::default())
      .await
      .unwrap();
    assert_eq!(got.feed_id.unwrap(), feed_id);
    assert_eq!(got.feed_status.unwrap(), "PROCESSED".to_string());
    assert_eq!(
      got.item_details.item_ingestion_status[0].mart_id.unwrap(),
      0,
    );
  }

  #[tokio::test]
  async fn test_bulk_upload_xml() {
    let client = crate::test_util::get_client_us();
    let body = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ns2:FeedAcknowledgement xmlns:ns2="http://walmart.com/">
    <ns2:feedId>884C20C71B7E42FAA41FABFA52596A62@AUoBAQA</ns2:feedId>
</ns2:FeedAcknowledgement>
    "#;

    let _m = mockito::mock("POST", "/v3/feeds")
      .match_query(Matcher::UrlEncoded("feedType".into(), "test".into()))
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let got = client
      .bulk_upload_xml("test", "somefeed".as_bytes())
      .await
      .unwrap();
    assert_eq!(
      got.feed_id.unwrap(),
      "884C20C71B7E42FAA41FABFA52596A62@AUoBAQA"
    );
  }
}
