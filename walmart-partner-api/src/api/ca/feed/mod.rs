use std::io::Read;

pub use types::*;

use crate::api::ca::Client;
use crate::client::Method;
use crate::result::WalmartResult;

mod types;

impl Client {
  pub async fn get_all_feed_statuses(
    &self,
    query: GetAllFeedStatusesQuery,
  ) -> WalmartResult<GetFeedStatuses> {
    let qs = serde_urlencoded::to_string(&query)?;
    let req = self.req_xml(Method::GET, "/v3/ca/feeds", qs)?;
    self.send(req).await?.res_xml().await
  }

  pub async fn get_feed_and_item_status(
    &self,
    feed_id: impl AsRef<str>,
    query: GetFeedAndItemStatusQuery,
  ) -> WalmartResult<GetFeedItemStatus> {
    let qs = serde_urlencoded::to_string(&query)?;
    let path = format!("/v3/ca/feeds/{}", feed_id.as_ref());
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
      .req_xml(Method::POST, "/v3/ca/feeds", vec![("feedType", feed_type)])?
      .form(form);
    self.send(req).await?.res_xml().await
  }
}

#[cfg(test)]
mod tests {
  use mockito::Matcher;

  use super::*;

  #[tokio::test]
  async fn get_all_feed_statuses() {
    let client = crate::test_util::get_client_ca();
    let body = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ns2:list xmlns:ns2="http://walmart.com/">
    <ns2:totalResults>1</ns2:totalResults>
    <ns2:offset>0</ns2:offset>
    <ns2:limit>50</ns2:limit>
    <ns2:results>
        <ns2:feed>
            <ns2:feedId>12234EGGT564YTEGFA@AQMBAQA</ns2:feedId>
            <ns2:feedSource>MARKETPLACE_PARTNER</ns2:feedSource>
            <ns2:feedType>item</ns2:feedType>
            <ns2:partnerId>1413254255</ns2:partnerId>
            <ns2:itemsReceived>1</ns2:itemsReceived>
            <ns2:itemsSucceeded>1</ns2:itemsSucceeded>
            <ns2:itemsFailed>0</ns2:itemsFailed>
            <ns2:itemsProcessing>0</ns2:itemsProcessing>
            <ns2:feedStatus>PROCESSED</ns2:feedStatus>
            <ns2:feedDate>2018-07-20T21:56:12.605Z</ns2:feedDate>
            <ns2:batchId>HP_REQUEST_BATCH</ns2:batchId>
            <ns2:modifiedDtm>2018-07-20T21:56:17.948Z</ns2:modifiedDtm>
            <ns2:fileName>ItemFeed99_ParadiseCounty_paperback.xml</ns2:fileName>
            <ns2:itemDataErrorCount>0</ns2:itemDataErrorCount>
            <ns2:itemSystemErrorCount>0</ns2:itemSystemErrorCount>
            <ns2:itemTimeoutErrorCount>0</ns2:itemTimeoutErrorCount>
            <ns2:channelType>WM_TEST</ns2:channelType>
        </ns2:feed>
    </ns2:results>
</ns2:list>"#;

    let feed_id = "12234EGGT564YTEGFA";
    let limit = 10;
    let _m = mockito::mock("GET", "/v3/ca/feeds")
      .match_query(Matcher::AllOf(vec![
        Matcher::UrlEncoded("feedId".into(), feed_id.into()),
        Matcher::UrlEncoded("limit".into(), limit.to_string()),
      ]))
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let q = GetAllFeedStatusesQuery {
      feed_id: Some(feed_id.to_string()),
      limit: Some(limit),
      offset: None,
    };
    let got = client.get_all_feed_statuses(q).await.unwrap();
    assert_eq!(got.total_results.unwrap(), 1);
    assert_eq!(got.offset.unwrap(), 0);
    assert_eq!(got.limit.unwrap(), 50);
    assert_eq!(
      got.results.feed[0].feed_id.as_ref().unwrap(),
      &"12234EGGT564YTEGFA@AQMBAQA".to_string()
    );
  }

  #[tokio::test]
  async fn get_feed_and_item_status() {
    let client = crate::test_util::get_client_ca();
    let body = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ns2:PartnerFeedResponse xmlns:ns2="http://walmart.com/">
    <ns2:feedId>640787F441ASSFF1C4FB7BB749E20C0A3</ns2:feedId>
    <ns2:feedStatus>PROCESSED</ns2:feedStatus>
    <ns2:feedSubmissionDate>2018-07-20T21:56:12.605Z</ns2:feedSubmissionDate>
    <ns2:itemsReceived>1</ns2:itemsReceived>
    <ns2:itemsSucceeded>1</ns2:itemsSucceeded>
    <ns2:itemsFailed>0</ns2:itemsFailed>
    <ns2:itemsProcessing>0</ns2:itemsProcessing>
    <ns2:offset>0</ns2:offset>
    <ns2:limit>50</ns2:limit>
    <ns2:itemDetails>
        <ns2:itemIngestionStatus>
            <ns2:martId>0</ns2:martId>
            <ns2:sku>234325346-8fbf-4fa0-a70c-2424rfwefq</ns2:sku>
            <ns2:wpid>7K69FC732QRRE5KTFS</ns2:wpid>
            <ns2:index>0</ns2:index>
            <ns2:itemid>24234</ns2:itemid>
            <ns2:productIdentifiers>
                <ns2:productIdentifier>
                    <ns2:productIdType>GTIN</ns2:productIdType>
                    <ns2:productId>086756453</ns2:productId>
                </ns2:productIdentifier>
                <ns2:productIdentifier>
                    <ns2:productIdType>ISBN</ns2:productIdType>
                    <ns2:productId>13432543634</ns2:productId>
                </ns2:productIdentifier>
            </ns2:productIdentifiers>
            <ns2:ingestionStatus>SUCCESS</ns2:ingestionStatus>
        </ns2:itemIngestionStatus>
    </ns2:itemDetails>
</ns2:PartnerFeedResponse>
    ""#;

    let feed_id = "640787F441ASSFF1C4FB7BB749E20C0A3";
    let _m = mockito::mock("GET", format!("/v3/ca/feeds/{}", feed_id).as_str())
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
    assert_eq!(got.item_details.item_ingestion_status.len(), 1);
    assert_eq!(
      got.item_details.item_ingestion_status[0].mart_id.unwrap(),
      0
    );
  }

  #[tokio::test]
  async fn test_bulk_upload_xml() {
    let client = crate::test_util::get_client_ca();
    let body = r#"
    <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<FeedAcknowledgement xmlns:ns2="http://walmart.com/">
    <feedId>E9C04D1FFD99479FBC1341D56DD5F930@AQMB_wA</feedId>
</FeedAcknowledgement>
    "#;

    let _m = mockito::mock("POST", "/v3/ca/feeds")
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
      "E9C04D1FFD99479FBC1341D56DD5F930@AQMB_wA"
    );
  }
}
