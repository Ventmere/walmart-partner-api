use serde::Serialize;

pub use types::*;

use crate::api::us::Client;
use crate::client::Method;
use crate::result::WalmartResult;

mod types;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllItemsQuery {
  pub next_cursor: Option<String>,
  pub sku: Option<String>,
  pub limit: Option<i32>,
  pub offset: Option<i32>,
  pub lifecycle_status: Option<String>,
  pub published_status: Option<String>,
  pub variant_group_id: Option<String>,
}

impl Client {
  pub async fn get_all_items(&self, query: GetAllItemsQuery) -> WalmartResult<GetAllItems> {
    let qs = serde_urlencoded::to_string(&query)?;
    let req = self.req_xml(Method::GET, "/v3/items", qs)?;
    self.send(req).await?.res_xml().await
  }

  pub async fn get_item(&self, sku: impl AsRef<str>) -> WalmartResult<GetItem> {
    let req = self.req_xml(Method::GET, &format!("/v3/items/{}", sku.as_ref()), "")?;
    self.send(req).await?.res_xml().await
  }

  pub async fn retire_item(&self, sku: impl AsRef<str>) -> WalmartResult<RetireItem> {
    let req = self.req_xml(Method::DELETE, &format!("/v3/items/{}", sku.as_ref()), "")?;
    self.send(req).await?.res_xml().await
  }
}

#[cfg(test)]
mod tests {
  use mockito::{mock, Matcher};

  use super::*;

  #[tokio::test]
  async fn test_get_all_items() {
    let client = crate::test_util::get_client_us();

    let q = GetAllItemsQuery {
      next_cursor: Some("foo".to_string()),
      ..Default::default()
    };

    let body = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ns2:ItemResponses xmlns:ns2="http://walmart.com/">
    <ns2:ItemResponse>
        <ns2:mart>WALMART_US</ns2:mart>
        <ns2:sku>2792005</ns2:sku>
        <ns2:wpid>0RE1TWYTKBKH</ns2:wpid>
        <ns2:upc>886859944807</ns2:upc>
        <ns2:gtin>00886859944807</ns2:gtin>
        <ns2:productName>Carhartt Men's Rugged Vest</ns2:productName>
        <ns2:shelf>[&quot;Home Page&quot;,&quot;Clothing&quot;,&quot;Mens Clothing&quot;,&quot;Mens Jackets &amp; Outerwear&quot;,&quot;Mens Jackets &amp; Outerwear&quot;]</ns2:shelf>
        <ns2:productType>Outerwear Coats, Jackets &amp; Vests</ns2:productType>
        <ns2:price>
            <ns2:currency>USD</ns2:currency>
            <ns2:amount>59.99</ns2:amount>
        </ns2:price>
        <ns2:publishedStatus>UNPUBLISHED</ns2:publishedStatus>
        <ns2:unpublishedReasons>
            <ns2:reason>No shipping information was set up. Please re - ingest this item with a shipping price if you are submitting an over - ride.</ns2:reason>
            <ns2:reason>You did not assign a tax code to your listing during item setup. Please re-ingest your item with a valid tax code.</ns2:reason>
        </ns2:unpublishedReasons>
        <ns2:lifecycleStatus>ACTIVE</ns2:lifecycleStatus>
    </ns2:ItemResponse>
    <ns2:ItemResponse>
        <ns2:mart>WALMART_US</ns2:mart>
        <ns2:sku>4534200</ns2:sku>
        <ns2:wpid>0RFL011ILHS3</ns2:wpid>
        <ns2:upc>889192592087</ns2:upc>
        <ns2:gtin>00889192592087</ns2:gtin>
        <ns2:productName>Carhartt Women's Slim-Fit Layton Skinny Leg Jean</ns2:productName>
        <ns2:shelf>[&quot;UNNAV&quot;]</ns2:shelf>
        <ns2:productType>Jeans</ns2:productType>
        <ns2:price>
            <ns2:currency>USD</ns2:currency>
            <ns2:amount>49.99</ns2:amount>
        </ns2:price>
        <ns2:publishedStatus>UNPUBLISHED</ns2:publishedStatus>
        <ns2:unpublishedReasons>
            <ns2:reason>No shipping information was set up. Please re - ingest this item with a shipping price if you are submitting an over - ride.</ns2:reason>
            <ns2:reason>You did not assign a tax code to your listing during item setup. Please re-ingest your item with a valid tax code.</ns2:reason>
        </ns2:unpublishedReasons>
        <ns2:lifecycleStatus>ACTIVE</ns2:lifecycleStatus>
    </ns2:ItemResponse>
    <ns2:ItemResponse>
        <ns2:mart>WALMART_US</ns2:mart>
        <ns2:sku>4419464</ns2:sku>
        <ns2:wpid>0RFUYQBZLPP5</ns2:wpid>
        <ns2:upc>889169324734</ns2:upc>
        <ns2:gtin>00889169324734</ns2:gtin>
        <ns2:productName>Marmot Women's Minimalist Jacket</ns2:productName>
        <ns2:shelf>[&quot;Home Page&quot;,&quot;Clothing&quot;,&quot;Women&quot;,&quot;Womens Coats &amp; Jackets Shop All&quot;]</ns2:shelf>
        <ns2:productType>Outerwear Coats, Jackets &amp; Vests</ns2:productType>
        <ns2:price>
            <ns2:currency>USD</ns2:currency>
            <ns2:amount>189.0</ns2:amount>
        </ns2:price>
        <ns2:publishedStatus>UNPUBLISHED</ns2:publishedStatus>
        <ns2:unpublishedReasons>
            <ns2:reason>No shipping information was set up. Please re - ingest this item with a shipping price if you are submitting an over - ride.</ns2:reason>
            <ns2:reason>You did not assign a tax code to your listing during item setup. Please re-ingest your item with a valid tax code.</ns2:reason>
        </ns2:unpublishedReasons>
        <ns2:lifecycleStatus>ACTIVE</ns2:lifecycleStatus>
    </ns2:ItemResponse>
    <ns2:ItemResponse>
        <ns2:mart>WALMART_US</ns2:mart>
        <ns2:sku>2928108</ns2:sku>
        <ns2:wpid>0RH410PGF6E5</ns2:wpid>
        <ns2:upc>883956217001</ns2:upc>
        <ns2:gtin>00883956217001</ns2:gtin>
        <ns2:productName>Olukai Women's Upena Sandal</ns2:productName>
        <ns2:shelf>[&quot;Home Page&quot;,&quot;Clothing&quot;,&quot;Shoes&quot;,&quot;Womens Shoes&quot;,&quot;Womens Sandals &amp; Flip-flops&quot;,&quot;Womens Sandals&quot;]</ns2:shelf>
        <ns2:productType>Sandals</ns2:productType>
        <ns2:price>
            <ns2:currency>USD</ns2:currency>
            <ns2:amount>89.95</ns2:amount>
        </ns2:price>
        <ns2:publishedStatus>UNPUBLISHED</ns2:publishedStatus>
        <ns2:unpublishedReasons>
            <ns2:reason>No shipping information was set up. Please re - ingest this item with a shipping price if you are submitting an over - ride.</ns2:reason>
            <ns2:reason>You did not assign a tax code to your listing during item setup. Please re-ingest your item with a valid tax code.</ns2:reason>
        </ns2:unpublishedReasons>
        <ns2:lifecycleStatus>ACTIVE</ns2:lifecycleStatus>
    </ns2:ItemResponse>
    <ns2:ItemResponse>
        <ns2:mart>WALMART_US</ns2:mart>
        <ns2:sku>4364805</ns2:sku>
        <ns2:wpid>0RXZMHYOF3YN</ns2:wpid>
        <ns2:upc>686487307056</ns2:upc>
        <ns2:gtin>00686487307056</ns2:gtin>
        <ns2:productName>Arcteryx Women's RHO LT Zip Neck Hooded</ns2:productName>
        <ns2:shelf>[&quot;UNNAV&quot;]</ns2:shelf>
        <ns2:productType>Sweatshirts &amp; Hoodies</ns2:productType>
        <ns2:price>
            <ns2:currency>USD</ns2:currency>
            <ns2:amount>139.0</ns2:amount>
        </ns2:price>
        <ns2:publishedStatus>SYSTEM_PROBLEM</ns2:publishedStatus>
        <ns2:unpublishedReasons>
            <ns2:reason>This item is prohibited because it violates one of our legal or compliance policies. For more details, create a case for Partner Support and include the code COMP.</ns2:reason>
        </ns2:unpublishedReasons>
        <ns2:lifecycleStatus>ACTIVE</ns2:lifecycleStatus>
    </ns2:ItemResponse>
    <ns2:totalItems>11440</ns2:totalItems>
    <ns2:nextCursor>AoE/GjBSWFpNSFlPRjNZTjBTRUxMRVJfT0ZGRVI1QUFDOTZFNzcyRjc0NkE1OTU5QjUxQTdGMUJFQTY5OQ==</ns2:nextCursor>
</ns2:ItemResponses>
    "#;

    let _m = mock("GET", "/v3/items")
      .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
        "nextCursor".into(),
        q.next_cursor.clone().unwrap(),
      )]))
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let got = client.get_all_items(q).await.unwrap();
    assert_eq!(got.item_response.len(), 5);
    let got = got.item_response[4].clone();
    let want = Item {
      mart: Some("WALMART_US".to_string()),
      sku: "4364805".to_string(),
      wpid: Some("0RXZMHYOF3YN".to_string()),
      upc: Some("686487307056".to_string()),
      gtin: Some("00686487307056".to_string()),
      product_name: Some("Arcteryx Women's RHO LT Zip Neck Hooded".to_string()),
      shelf: Some("[\"UNNAV\"]".to_string()),
      product_type: Some("Sweatshirts & Hoodies".to_string()),
      price: Some(Price {
        currency: "USD".to_string(),
        amount: "139.0".to_string(),
      }),
      published_status: Some("SYSTEM_PROBLEM".to_string()),
    };
    assert_eq!(got, want);
  }

  #[tokio::test]
  async fn test_get_item() {
    let client = crate::test_util::get_client_us();

    let sku = "foo";
    let body = r#"
<?xml version="1.0" encoding="UTF-8"?>
<ns2:ItemResponses xmlns:ns2="http://walmart.com/">
<ns2:ItemResponse>
        <ns2:mart>WALMART_US</ns2:mart>
        <ns2:sku>setup_by_ref</ns2:sku>
        <ns2:wpid>2VYRD2YCHYX1</ns2:wpid>
        <ns2:gtin>05518319011365</ns2:gtin>
        <ns2:productName>WL_Sim_verizon</ns2:productName>
        <ns2:productType>Music</ns2:productType>
        <ns2:price>
            <ns2:currency>USD</ns2:currency>
            <ns2:amount>12.00</ns2:amount>
        </ns2:price>
        <ns2:publishedStatus>UNPUBLISHED</ns2:publishedStatus>
        <ns2:unpublishedDescription>
            Your item is unpublished because the end date has passed. To republish your item, re-ingest the item with a new start and end date.
        </ns2:unpublishedDescription>
        <ns2:lifecycleStatus>RETIRED</ns2:lifecycleStatus>
</ns2:ItemResponse>
</ns2:ItemResponses>
    "#;

    let _m = mock("GET", format!("/v3/items/{}", sku).as_str())
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let got = client.get_item(sku).await.unwrap();
    let want = GetItem {
      item_response: Item {
        mart: Some("WALMART_US".to_string()),
        sku: "setup_by_ref".to_string(),
        wpid: Some("2VYRD2YCHYX1".into()),
        upc: None,
        gtin: Some("05518319011365".into()),
        product_name: Some("WL_Sim_verizon".into()),
        shelf: None,
        product_type: Some("Music".into()),
        price: Some(Price {
          currency: "USD".to_string(),
          amount: "12.00".into(),
        }),
        published_status: Some("UNPUBLISHED".into()),
      },
    };
    assert_eq!(got, want);
  }

  #[tokio::test]
  async fn test_retire_item() {
    let client = crate::test_util::get_client_us();

    let sku = "foo";
    let body = r#"
    <?xml version="1.0" encoding="UTF-8"?>
<ItemRetireResponse xmlns:ns2="http://walmart.com/">
    <sku>34931712</sku>
    <message>Thank you. Your item has been submitted for retirement from Walmart Catalog. Please note that it can take up to 48 hours for items to be retired from our catalog.</message>
</ItemRetireResponse>
    "#;

    let _m = mock("DELETE", format!("/v3/items/{}", sku).as_str())
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
