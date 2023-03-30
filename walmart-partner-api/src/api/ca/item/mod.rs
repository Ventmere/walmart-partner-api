use serde::Serialize;

pub use types::*;

use crate::api::ca::Client;
use crate::client::Method;
use crate::result::WalmartResult;

mod types;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllItemsQuery {
  pub next_cursor: GetAllItemsCursor,
  pub sku: Option<String>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetAllItemsCursor(String);

impl Default for GetAllItemsCursor {
  fn default() -> Self {
    GetAllItemsCursor("*".to_string())
  }
}

impl<T: ToString> From<T> for GetAllItemsCursor {
  fn from(s: T) -> Self {
    GetAllItemsCursor(s.to_string())
  }
}

impl Into<String> for GetAllItemsCursor {
  fn into(self) -> String {
    self.0
  }
}

impl Client {
  pub async fn get_all_items(&self, query: GetAllItemsQuery) -> WalmartResult<GetAllItems> {
    let qs = serde_urlencoded::to_string(&query)?;
    let req = self.req_xml(Method::GET, "/v3/ca/items", qs)?;
    self.send(req).await?.res_xml().await
  }

  pub async fn get_item(&self, sku: impl AsRef<str>) -> WalmartResult<GetItem> {
    let req = self.req_xml(Method::GET, &format!("/v3/ca/items/{}", sku.as_ref()), "")?;
    self.send(req).await?.res_xml().await
  }

  pub async fn retire_item(&self, sku: impl AsRef<str>) -> WalmartResult<RetireItem> {
    let req = self.req_xml(
      Method::DELETE,
      &format!("/v3/ca/items/{}", sku.as_ref()),
      "",
    )?;
    self.send(req).await?.res_xml().await
  }
}

#[cfg(test)]
mod tests {
  use mockito::{mock, Matcher};

  use super::*;

  #[tokio::test]
  async fn test_get_all_items() {
    let client = crate::test_util::get_client_ca();

    let q = GetAllItemsQuery {
      ..Default::default()
    };

    let body = r#"
    <?xml version="1.0" encoding="UTF-8"?>
<ItemResponses xmlns:ns2="http://walmart.com/">
    <ItemResponse>
        <mart>WALMART_CA</mart>
        <sku>379third908</sku>
        <gtin>00313159099947</gtin>
        <productName>Carex Soft Grip Folding Cane - Black Walking Cane</productName>
        <productType>Walking Canes</productType>
        <price>
            <currency>CAD</currency>
            <amount>13.27</amount>
        </price>
        <publishedStatus>IN_PROGRESS</publishedStatus>
    </ItemResponse>
    <ItemResponse>
        <mart>WALMART_US</mart>
        <sku>prodtest1571</sku>
        <upc>889296686590</upc>
        <gtin>00889296686590</gtin>
        <productName>REFURBISHED: HP 250 G3 15.6" Notebook, Intel 4th Gen i3, 4GB RAM, 500GB HDD, Win 8.1, M5G69UT#ABA</productName>
        <productType>RAM Memory</productType>
        <price>
            <currency>CAD</currency>
            <amount>329.99</amount>
        </price>
        <publishedStatus>PUBLISHED</publishedStatus>
    </ItemResponse>
</ItemResponses>"#;

    let _m = mock("GET", "/v3/ca/items")
      .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
        "nextCursor".into(),
        q.next_cursor.clone().into(),
      )]))
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let items = client.get_all_items(q).await.unwrap();
    let got = items.item_response[0].clone();
    let want = Item {
      mart: Some("WALMART_CA".to_string()),
      sku: "379third908".to_string(),
      wpid: None,
      upc: None,
      gtin: Some("00313159099947".to_string()),
      product_name: Some("Carex Soft Grip Folding Cane - Black Walking Cane".to_string()),
      shelf: None,
      product_type: Some("Walking Canes".to_string()),
      price: Some(Price {
        currency: "CAD".to_string(),
        amount: "13.27".to_string(),
      }),
      published_status: Some("IN_PROGRESS".to_string()),
    };
    assert_eq!(got, want);
  }

  #[tokio::test]
  async fn test_get_item() {
    let client = crate::test_util::get_client_ca();

    let sku = "foo";
    let body = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ItemResponses>
  <ItemResponse>
    <mart>WALMART_CA</mart>
    <wpid>4W1AF6YU04F5</wpid>
    <sku>foo</sku>
    <upc>735732770692</upc>
    <gtin>00735732770692</gtin>
    <productName>Victoria Classics Vista Paisley 8-Piece Bedding Comforter Set</productName>
    <productType>Bedding Sets</productType>
    <price>
      <currency>CAD</currency>
      <amount>13.27</amount>
    </price>
    <publishedStatus>IN_PROGRESS</publishedStatus>
  </ItemResponse>
</ItemResponses>
    "#;

    let _m = mock("GET", format!("/v3/ca/items/{}", sku).as_str())
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let got = client.get_item(sku).await.unwrap();
    let want = GetItem {
      item_response: Item {
        mart: Some("WALMART_CA".to_string()),
        sku: "foo".to_string(),
        wpid: Some("4W1AF6YU04F5".into()),
        upc: Some("735732770692".into()),
        gtin: Some("00735732770692".into()),
        product_name: Some("Victoria Classics Vista Paisley 8-Piece Bedding Comforter Set".into()),
        shelf: None,
        product_type: Some("Bedding Sets".into()),
        price: Some(Price {
          currency: "CAD".to_string(),
          amount: "13.27".into(),
        }),
        published_status: Some("IN_PROGRESS".into()),
      },
    };
    assert_eq!(got, want);
  }

  #[tokio::test]
  async fn test_retire_item() {
    let client = crate::test_util::get_client_ca();

    let sku = "foo";
    let body = r#"
    <?xml version="1.0" encoding="UTF-8"?>
<ItemRetireResponse xmlns:ns2="http://walmart.com/">
    <sku>34931712</sku>
    <message>Thank you. Your item has been submitted for retirement from Walmart Catalog. Please note that it can take up to 48 hours for items to be retired from our catalog.</message>
</ItemRetireResponse>
    "#;

    let _m = mock("DELETE", format!("/v3/ca/items/{}", sku).as_str())
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let got = client.retire_item(sku).await.unwrap();
    let want = RetireItem {
      sku: "34931712".to_string(),
      message: Some("Thank you. Your item has been submitted for retirement from Walmart Catalog. Please note that it can take up to 48 hours for items to be retired from our catalog.".to_string())
    };
    assert_eq!(got, want);
  }
}
