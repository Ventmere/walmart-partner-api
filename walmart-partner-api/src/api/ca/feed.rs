use crate::ca::Client;
use crate::client::Method;
pub use crate::feed::{
  FeedStatuses, GetAllFeedStatusesQuery, GetFeedAndItemStatusQuery, PartnerFeedResponse,
};
use crate::response::ResponseExt;
use crate::result::WalmartResult;

impl Client {
  pub async fn get_all_feed_statuses(
    &self,
    query: &GetAllFeedStatusesQuery,
  ) -> WalmartResult<FeedStatuses> {
    let qs = serde_urlencoded::to_string(query)?;
    let req = self.req_xml(Method::GET, "/v3/ca/feeds", qs, None)?;
    self.send(req).await?.xml::<FeedStatuses>().await
  }

  // pub async fn get_feed_and_item_status(
  //   &self,
  //   feed_id: impl AsRef<str>,
  //   query: &GetFeedAndItemStatusQuery,
  // ) -> WalmartResult<PartnerFeedResponse> {
  //   let qs = serde_urlencoded::to_string(query)?;
  //   let path = format!("/v3/ca/feeds/{}", feed_id.as_ref());
  //   let req = self.req_xml(Method::GET, &path, qs)?;
  //   self
  //     .send(req)?
  //     .xml_res::<PartnerFeedResponse>()
  //     .map_err(Into::into)
  // }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_deserialize_get_all_feed_statuses_xml() {
    let xml = r##"
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
    </ns2:list>"##;
    quick_xml::de::from_str::<FeedStatuses>(xml).unwrap();
  }

  #[test]
  fn test_deserialize_get() {
    let xml = r##"
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
    "##;
    quick_xml::de::from_str::<PartnerFeedResponse>(xml).unwrap();
  }
}
