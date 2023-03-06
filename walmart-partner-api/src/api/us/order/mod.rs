use chrono::{DateTime, Utc};
use serde::Serialize;
use xml_builder::XMLElement;

pub use types::*;

use crate::api::us::Client;
use crate::client::Method;
use crate::{WalmartResult, XmlSer};

mod types;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllReleasedOrdersQuery {
  pub limit: Option<i32>,
  pub created_start_date: Option<DateTime<Utc>>,
  pub created_end_date: Option<DateTime<Utc>>,
  pub product_info: Option<bool>,
  pub sku: Option<String>,
  pub customer_order_id: Option<String>,
  pub purchase_order_id: Option<String>,
  pub from_expected_ship_date: Option<DateTime<Utc>>,
  pub to_expected_ship_date: Option<DateTime<Utc>>,
  pub replacement_into: Option<bool>,
  pub order_type: Option<String>,
  pub ship_node_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllOrdersQuery {
  pub sku: Option<String>,
  pub customer_order_id: Option<String>,
  pub purchase_order_id: Option<String>,
  pub status: Option<String>,
  pub created_start_date: Option<DateTime<Utc>>,
  pub created_end_date: Option<DateTime<Utc>>,
  pub from_expected_ship_date: Option<DateTime<Utc>>,
  pub to_expected_ship_date: Option<DateTime<Utc>>,
  pub limit: Option<i32>,
  pub product_info: Option<bool>,
  pub ship_node_type: Option<String>,
  pub replacement_into: Option<bool>,
  pub order_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrderQuery {
  pub product_info: Option<bool>,
  pub replacement_into: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ShipOrderLines {
  pub order_lines: Vec<ShipOrderLine>,
  pub process_mode: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ShipOrderLine {
  pub line_number: String,
  pub seller_order_id: String,
  pub intent_to_cancel_override: Option<bool>,
  /// If not provided the default values are used
  pub status_quantity: Option<ShipOrderLineStatusQuantity>,
  pub asn: Option<ShipOrderLineAsn>,
  pub tracking_info: OrderLineTrackingInfo,
}

impl Client {
  pub async fn get_all_orders(&self, query: GetAllOrdersQuery) -> WalmartResult<OrderList> {
    let qs = serde_urlencoded::to_string(query)?;
    let req = self.req_xml(Method::GET, "/v3/orders", qs)?;
    self.send(req).await?.res_xml().await
  }

  pub async fn get_all_orders_by_next_cursor(
    &self,
    next_cursor: impl AsRef<str>,
  ) -> WalmartResult<OrderList> {
    use url::form_urlencoded;
    let req = self.req_xml(
      Method::GET,
      "/v3/orders",
      form_urlencoded::parse((&next_cursor.as_ref()[1..]).as_bytes())
        .into_owned()
        .collect::<Vec<_>>(),
    )?;
    self.send(req).await?.res_xml().await
  }

  pub async fn get_all_released_orders(
    &self,
    query: GetAllReleasedOrdersQuery,
  ) -> WalmartResult<OrderList> {
    let qs = serde_urlencoded::to_string(query)?;
    let req = self.req_xml(Method::GET, "/v3/orders/released", qs)?;
    self.send(req).await?.res_xml().await
  }

  pub async fn get_order(
    &self,
    purchase_order_id: impl AsRef<str>,
    query: GetOrderQuery,
  ) -> WalmartResult<Order> {
    let qs = serde_urlencoded::to_string(query)?;
    let req = self.req_xml(
      Method::GET,
      &format!("/v3/orders/{}", purchase_order_id.as_ref()),
      qs,
    )?;
    self.send(req).await?.res_xml().await
  }

  pub async fn ack_order(&self, purchase_order_id: impl AsRef<str>) -> WalmartResult<Order> {
    let req = self.req_xml(
      Method::POST,
      &format!("/v3/orders/{}/acknowledge", purchase_order_id.as_ref()),
      (),
    )?;
    self.send(req).await?.res_xml().await
  }

  pub async fn ship_order_lines(
    &self,
    purchase_order_id: impl AsRef<str>,
    input: ShipOrderLines,
  ) -> WalmartResult<Order> {
    let req = self
      .req_xml(
        Method::POST,
        &format!("/v3/orders/{}/shipping", purchase_order_id.as_ref()),
        (),
      )?
      .body_xml(input)?;

    self.send(req).await?.res_xml().await
  }
}

impl XmlSer for ShipOrderLines {
  fn to_xml(&self) -> WalmartResult<XMLElement> {
    let mut order_shipment = XMLElement::new("orderShipment");
    order_shipment.add_attribute("xmlns", "http://walmart.com/mp/v3/orders");
    let mut order_lines = XMLElement::new("orderLines");

    if let Some(process_mode_v) = self.process_mode.clone() {
      let mut process_mode = XMLElement::new("processMode");
      process_mode.add_text(process_mode_v)?;
      order_shipment.add_child(process_mode)?;
    }

    for line in &self.order_lines {
      order_lines.add_child(line.to_xml()?)?;
    }

    order_shipment.add_child(order_lines)?;
    Ok(order_shipment)
  }
}

impl XmlSer for ShipOrderLine {
  fn to_xml(&self) -> WalmartResult<XMLElement> {
    let mut order_line = XMLElement::new("orderLine");
    let mut line_number = XMLElement::new("lineNumber");
    line_number.add_text(self.line_number.clone())?;
    order_line.add_child(line_number)?;

    let mut seller_order_id = XMLElement::new("sellerOrderId");
    seller_order_id.add_text(self.seller_order_id.clone())?;
    order_line.add_child(seller_order_id)?;

    if let Some(intent_to_cancel_override_v) = self.intent_to_cancel_override.clone() {
      let mut intent_to_cancel_override = XMLElement::new("intentToCancelOverride");
      intent_to_cancel_override.add_text(intent_to_cancel_override_v.to_string())?;
      order_line.add_child(intent_to_cancel_override)?;
    }

    let mut order_line_statuses = XMLElement::new("orderLineStatuses");
    let mut order_line_status = XMLElement::new("orderLineStatus");
    {
      let mut status = XMLElement::new("status");
      status.add_text("Shipped".to_string())?;
      order_line_status.add_child(status)?;

      if let Some(asn) = self.asn.clone() {
        order_line_status.add_child(asn.to_xml()?)?;
      }

      order_line_status.add_child(self.status_quantity.to_xml()?)?;
      order_line_status.add_child(self.tracking_info.to_xml()?)?;
    }
    order_line_statuses.add_child(order_line_status)?;
    order_line.add_child(order_line_statuses)?;

    Ok(order_line)
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use mockito::{mock, Matcher};

  use super::*;

  #[tokio::test]
  async fn test_get_all_orders() {
    let client = crate::test_util::get_client_us();

    let _m = mock("GET", "/v3/orders")
      .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
        "limit".into(),
        "10".into(),
      )]))
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(include_str!("./test_get_all_orders_body.xml"))
      .create();

    let got = client
      .get_all_orders(GetAllOrdersQuery {
        limit: Some(10),
        ..Default::default()
      })
      .await
      .unwrap();

    assert_eq!(got.elements.order.len(), 10);
    let order = got.elements.order[0].clone();
    assert_eq!(order.purchase_order_id, "1796277083022");
    let date = DateTime::<Utc>::from_str("2019-09-14T13:09:31.000Z").unwrap();
    assert_eq!(order.order_date, date);
    assert_eq!(order.shipping_info.method_code, "Value");
    assert_eq!(
      order.shipping_info.postal_address.address1,
      "3258BWarners rd".to_string()
    );
    assert_eq!(order.order_lines.order_line.len(), 1);
    let line = order.order_lines.order_line[0].clone();
    assert_eq!(line.line_number, 4.to_string());
    assert_eq!(line.charges.charge.len(), 1);
  }

  #[tokio::test]
  async fn test_get_all_released_orders() {
    let client = crate::test_util::get_client_us();

    let _m = mock("GET", "/v3/orders/released")
      .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
        "limit".into(),
        "10".into(),
      )]))
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(include_str!("./test_get_all_released_order_body.xml"))
      .create();

    let got = client
      .get_all_released_orders(GetAllReleasedOrdersQuery {
        limit: Some(10),
        ..Default::default()
      })
      .await
      .unwrap();
    assert_eq!(got.meta.total_count, Some(78449));
    assert_eq!(got.elements.order.len(), 10);
  }

  #[tokio::test]
  async fn test_get_order() {
    let client = crate::test_util::get_client_us();
    let body = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ns3:order xmlns:ns2="http://walmart.com/mp/orders" xmlns:ns3="http://walmart.com/mp/v3/orders" xmlns:ns4="com.walmart.services.common.model.money" xmlns:ns5="com.walmart.services.common.model.name" xmlns:ns6="com.walmart.services.common.model.address" xmlns:ns7="com.walmart.services.common.model.address.validation" xmlns:ns8="http://walmart.com/">
   <ns3:purchaseOrderId>1815367951431</ns3:purchaseOrderId>
   <ns3:customerOrderId>4372135469400</ns3:customerOrderId>
   <ns3:customerEmailId>7D9F02968955480499C84C5871AD1401@relay.walmart.com</ns3:customerEmailId>
   <ns3:orderDate>2021-06-15T22:33:27.000Z</ns3:orderDate>
   <ns3:shippingInfo>
      <ns3:phone>3143450432</ns3:phone>
      <ns3:estimatedDeliveryDate>2021-06-25T19:00:00.000Z</ns3:estimatedDeliveryDate>
      <ns3:estimatedShipDate>2021-06-17T05:00:00.000Z</ns3:estimatedShipDate>
      <ns3:methodCode>Value</ns3:methodCode>
      <ns3:postalAddress>
         <ns3:name>Donald Ashley</ns3:name>
         <ns3:address1>6600 gazebo drive</ns3:address1>
         <ns3:city>Cedar hill</ns3:city>
         <ns3:state>MO</ns3:state>
         <ns3:postalCode>63016</ns3:postalCode>
         <ns3:country>USA</ns3:country>
         <ns3:addressType>RESIDENTIAL</ns3:addressType>
      </ns3:postalAddress>
   </ns3:shippingInfo>
   <ns3:orderLines>
      <ns3:orderLine>
         <ns3:lineNumber>1</ns3:lineNumber>
         <ns3:item>
            <ns3:productName>Mac Sports Heavy Duty Steel Double Decker Collapsible Yard Cart Wagon, Purple</ns3:productName>
            <ns3:sku>YHY-FN25200B2</ns3:sku>
         </ns3:item>
         <ns3:charges>
            <ns3:charge>
               <ns3:chargeType>PRODUCT</ns3:chargeType>
               <ns3:chargeName>ItemPrice</ns3:chargeName>
               <ns3:chargeAmount>
                  <ns3:currency>USD</ns3:currency>
                  <ns3:amount>5.00</ns3:amount>
               </ns3:chargeAmount>
               <ns3:tax>
                  <ns3:taxName>Tax1</ns3:taxName>
                  <ns3:taxAmount>
                     <ns3:currency>USD</ns3:currency>
                     <ns3:amount>0.34</ns3:amount>
                  </ns3:taxAmount>
               </ns3:tax>
            </ns3:charge>
         </ns3:charges>
         <ns3:orderLineQuantity>
            <ns3:unitOfMeasurement>EACH</ns3:unitOfMeasurement>
            <ns3:amount>1</ns3:amount>
         </ns3:orderLineQuantity>
         <ns3:statusDate>2021-06-16T09:30:55.623Z</ns3:statusDate>
         <ns3:orderLineStatuses>
            <ns3:orderLineStatus>
               <ns3:status>Shipped</ns3:status>
               <ns3:statusQuantity>
                  <ns3:unitOfMeasurement>EACH</ns3:unitOfMeasurement>
                  <ns3:amount>1</ns3:amount>
               </ns3:statusQuantity>
               <ns3:trackingInfo>
                  <ns3:shipDateTime>2021-06-16T09:30:55.000Z</ns3:shipDateTime>
                  <ns3:carrierName>
                     <ns3:carrier>USPS</ns3:carrier>
                  </ns3:carrierName>
                  <ns3:methodCode>Value</ns3:methodCode>
                  <ns3:trackingNumber>435678956435467</ns3:trackingNumber>
                  <ns3:trackingURL>https://www.walmart.com/tracking?tracking_id=435678956435467&amp;order_id=1815367951431</ns3:trackingURL>
               </ns3:trackingInfo>
            </ns3:orderLineStatus>
         </ns3:orderLineStatuses>
         <ns3:fulfillment>
            <ns3:fulfillmentOption>S2H</ns3:fulfillmentOption>
            <ns3:shipMethod>VALUE</ns3:shipMethod>
            <ns3:pickUpDateTime>2021-06-25T19:00:00.000Z</ns3:pickUpDateTime>
         </ns3:fulfillment>
      </ns3:orderLine>
   </ns3:orderLines>
   <ns3:shipNode>
      <ns3:type>SellerFulfilled</ns3:type>
   </ns3:shipNode>
</ns3:order>
    "#;

    let _m = mock("GET", "/v3/orders/1796330120075")
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let got = client
      .get_order("1796330120075", Default::default())
      .await
      .unwrap();
    assert_eq!(got.purchase_order_id, "1815367951431");
    assert_eq!(got.customer_order_id, "4372135469400");
    assert_eq!(got.order_lines.order_line.len(), 1);
  }

  #[tokio::test]
  pub async fn test_ack_order() {
    let client = crate::test_util::get_client_us();
    let body = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ns3:order xmlns:ns2="http://walmart.com/mp/orders" xmlns:ns3="http://walmart.com/mp/v3/orders" xmlns:ns4="com.walmart.services.common.model.money" xmlns:ns5="com.walmart.services.common.model.name" xmlns:ns6="com.walmart.services.common.model.address" xmlns:ns7="com.walmart.services.common.model.address.validation" xmlns:ns8="http://walmart.com/">
    <ns3:purchaseOrderId>1796277083022</ns3:purchaseOrderId>
    <ns3:customerOrderId>5281956426648</ns3:customerOrderId>
    <ns3:customerEmailId>3A31739D8B0A45A1B23F7F8C81C8747F@relay.walmart.com</ns3:customerEmailId>
    <ns3:orderDate>2019-09-14T13:09:31.000Z</ns3:orderDate>
    <ns3:shippingInfo>
        <ns3:phone>3155598681</ns3:phone>
        <ns3:estimatedDeliveryDate>2019-09-25T19:00:00.000Z</ns3:estimatedDeliveryDate>
        <ns3:estimatedShipDate>2019-09-17T06:00:00.000Z</ns3:estimatedShipDate>
        <ns3:methodCode>Value</ns3:methodCode>
        <ns3:postalAddress>
            <ns3:name>Kathryn Cole</ns3:name>
            <ns3:address1>3258BWarners rd</ns3:address1>
            <ns3:address2>Garage</ns3:address2>
            <ns3:city>Warners</ns3:city>
            <ns3:state>NY</ns3:state>
            <ns3:postalCode>13164</ns3:postalCode>
            <ns3:country>USA</ns3:country>
            <ns3:addressType>RESIDENTIAL</ns3:addressType>
        </ns3:postalAddress>
    </ns3:shippingInfo>
    <ns3:orderLines>
        <ns3:orderLine>
            <ns3:lineNumber>4</ns3:lineNumber>
            <ns3:item>
                <ns3:productName>Beba Bean Pee-pee Teepee Airplane - Blue - Laundry Bag</ns3:productName>
                <ns3:sku>test1</ns3:sku>
            </ns3:item>
            <ns3:charges>
                <ns3:charge>
                    <ns3:chargeType>PRODUCT</ns3:chargeType>
                    <ns3:chargeName>ItemPrice</ns3:chargeName>
                    <ns3:chargeAmount>
                        <ns3:currency>USD</ns3:currency>
                        <ns3:amount>10.00</ns3:amount>
                    </ns3:chargeAmount>
                    <ns3:tax>
                        <ns3:taxName>Tax1</ns3:taxName>
                        <ns3:taxAmount>
                            <ns3:currency>USD</ns3:currency>
                            <ns3:amount>0.80</ns3:amount>
                        </ns3:taxAmount>
                    </ns3:tax>
                </ns3:charge>
            </ns3:charges>
            <ns3:orderLineQuantity>
                <ns3:unitOfMeasurement>EACH</ns3:unitOfMeasurement>
                <ns3:amount>1</ns3:amount>
            </ns3:orderLineQuantity>
            <ns3:statusDate>2019-09-17T20:45:56.000Z</ns3:statusDate>
            <ns3:orderLineStatuses>
                <ns3:orderLineStatus>
                    <ns3:status>Acknowledged</ns3:status>
                    <ns3:statusQuantity>
                        <ns3:unitOfMeasurement>EACH</ns3:unitOfMeasurement>
                        <ns3:amount>1</ns3:amount>
                    </ns3:statusQuantity>
                </ns3:orderLineStatus>
            </ns3:orderLineStatuses>
            <ns3:fulfillment>
                <ns3:fulfillmentOption>S2H</ns3:fulfillmentOption>
                <ns3:shipMethod>VALUE</ns3:shipMethod>
                <ns3:pickUpDateTime>2019-09-19T19:00:00.000Z</ns3:pickUpDateTime>
            </ns3:fulfillment>
        </ns3:orderLine>
    </ns3:orderLines>
</ns3:order>
    "#;

    let _m = mock("POST", "/v3/orders/2575193093772/acknowledge")
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let got = client.ack_order("2575193093772").await.unwrap();
    assert_eq!(got.shipping_info.phone, "3155598681".to_string());
    assert_eq!(got.order_lines.order_line.len(), 1);
  }

  #[tokio::test]
  pub async fn test_ship_order_serialize() {
    struct TestCase {
      name: &'static str,
      input: ShipOrderLines,
      want: &'static str,
    }

    let test_cases = vec![TestCase {
      name: "full_input",
      input: ShipOrderLines {
        process_mode: Some("PARTIAL_UPDATE".to_string()),
        order_lines: vec![ShipOrderLine {
          line_number: "2".to_string(),
          seller_order_id: "seller_id".to_string(),
          intent_to_cancel_override: Some(true),
          status_quantity: Some(ShipOrderLineStatusQuantity {
            unit_of_measurement: Some("EACH".to_string()),
            amount: Some(1.to_string()),
          }),
          asn: Some(ShipOrderLineAsn {
            package_asn: "package_asn".to_string(),
            pallet_asn: Some("pallet_asn".to_string()),
          }),
          tracking_info: OrderLineTrackingInfo {
            ship_date_time: DateTime::<Utc>::from_str("2016-06-27T05:30:15.000Z").unwrap(),
            carrier_name: OrderLineTrackingCarrier {
              other_carrier: None,
              carrier: Some("FedEx".to_string()),
            },
            method_code: "Standard".to_string(),
            tracking_number: "12333634122".to_string(),
            tracking_url: Some("http://www.fedex.com".to_string()),
          },
        }],
      },
      want: r#"
<orderShipment
  xmlns="http://walmart.com/mp/v3/orders">
  <processMode>PARTIAL_UPDATE</processMode>
    <orderLines>
        <orderLine>
            <lineNumber>2</lineNumber>
            <sellerOrderId>seller_id</sellerOrderId>
            <intentToCancelOverride>true</intentToCancelOverride>
            <orderLineStatuses>
                <orderLineStatus>
                    <status>Shipped</status>
                    <asn>
                        <packageASN>package_asn</packageASN>
                        <palletASN>pallet_asn</palletASN>
                    </asn>
                    <statusQuantity>
                        <unitOfMeasurement>EACH</unitOfMeasurement>
                        <amount>1</amount>
                    </statusQuantity>
                    <trackingInfo>
                        <shipDateTime>2016-06-27T05:30:15+00:00</shipDateTime>
                        <carrierName>
                            <carrier>FedEx</carrier>
                        </carrierName>
                        <methodCode>Standard</methodCode>
                        <trackingNumber>12333634122</trackingNumber>
                        <trackingURL>http://www.fedex.com</trackingURL>
                    </trackingInfo>
                </orderLineStatus>
            </orderLineStatuses>
        </orderLine>
    </orderLines>
</orderShipment>"#,
    }];

    for tc in test_cases {
      crate::test_util::assert_xml_eq(tc.input, tc.want, format!("test case: {}", tc.name));
    }
  }

  #[tokio::test]
  pub async fn test_ship_order_lines() {
    let client = crate::test_util::get_client_us();
    let body = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes" ?>
<order
        xmlns:ns2="http://walmart.com/mp/orders"
        xmlns:ns3="http://walmart.com/mp/v3/orders"
        xmlns:ns4="http://walmart.com/">
    <purchaseOrderId>2575193093772</purchaseOrderId>
    <customerOrderId>4021603841173</customerOrderId>
    <customerEmailId>mgr@walmartlabs.com</customerEmailId>
    <orderDate>2016-05-11T22:55:28.000Z</orderDate>
    <shippingInfo>
        <phone>6502248603</phone>
        <estimatedDeliveryDate>2016-05-20T17:00:00.000Z</estimatedDeliveryDate>
        <estimatedShipDate>2016-05-16T17:00:00.000Z</estimatedShipDate>
        <methodCode>Standard</methodCode>
        <postalAddress>
            <name>Joe Doe PGOMS</name>
            <address1>860 W Cal Ave</address1>
            <address2>Seat # 860C.2.176</address2>
            <city>Sunnyvale</city>
            <state>CA</state>
            <postalCode>94086</postalCode>
            <country>USA</country>
            <addressType>RESIDENTIAL</addressType>
        </postalAddress>
    </shippingInfo>
    <orderLines>
        <orderLine>
            <lineNumber>2</lineNumber>
            <item>
                <productName>Garmin Refurbished nuvi 2595LMT 5 GPS w Lifetime Maps and Traffic</productName>
                <sku>GRMN100201</sku>
            </item>
            <charges>
                <charge>
                    <chargeType>PRODUCT</chargeType>
                    <chargeName>ItemPrice</chargeName>
                    <chargeAmount>
                        <currency>USD</currency>
                        <amount>124.98</amount>
                    </chargeAmount>
                    <tax>
                        <taxName>Tax1</taxName>
                        <taxAmount>
                            <currency>USD</currency>
                            <amount>10.93</amount>
                        </taxAmount>
                    </tax>
                </charge>
            </charges>
            <orderLineQuantity>
                <unitOfMeasurement>EACH</unitOfMeasurement>
                <amount>1</amount>
            </orderLineQuantity>
            <statusDate>2016-06-03T23:44:41.000Z</statusDate>
            <orderLineStatuses>
                <orderLineStatus>
                    <status>Shipped</status>
                    <statusQuantity>
                        <unitOfMeasurement>EACH</unitOfMeasurement>
                        <amount>1</amount>
                    </statusQuantity>
                    <trackingInfo>
                        <shipDateTime>2016-06-27T05:30:15.000Z</shipDateTime>
                        <carrierName>
                            <carrier>FedEx</carrier>
                        </carrierName>
                        <methodCode>Standard</methodCode>
                        <trackingNumber>12333634122</trackingNumber>
                        <trackingURL>http://www.fedex.com</trackingURL>
                    </trackingInfo>
                </orderLineStatus>
            </orderLineStatuses>
        </orderLine>
    </orderLines>
</order>
    "#;

    let _m = mock("POST", "/v3/orders/2575193093772/shipping")
      .with_status(200)
      .with_header("content-type", "application/xml")
      .match_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let input = ShipOrderLines {
      order_lines: vec![ShipOrderLine {
        line_number: "2".to_string(),
        seller_order_id: "seller_order_id".to_string(),
        intent_to_cancel_override: Some(true),
        status_quantity: Some(ShipOrderLineStatusQuantity {
          unit_of_measurement: Some("EA".to_string()),
          amount: Some("2".to_string()),
        }),
        asn: Some(ShipOrderLineAsn {
          package_asn: "package_asn".to_string(),
          pallet_asn: Some("pallet_asn".to_string()),
        }),
        tracking_info: OrderLineTrackingInfo {
          ship_date_time: DateTime::<Utc>::from_str("2016-06-27T05:30:15+00:00").unwrap(),
          carrier_name: OrderLineTrackingCarrier {
            other_carrier: None,
            carrier: Some("FedEx".to_string()),
          },
          method_code: "Standard".to_string(),
          tracking_number: "12333634122".to_string(),
          tracking_url: Some("http://www.fedex.com".to_string()),
        },
      }],
      process_mode: Some("PARTIAL_UPDATE".to_string()),
    };

    let got = client
      .ship_order_lines("2575193093772", input)
      .await
      .unwrap();
    assert_eq!(got.purchase_order_id, "2575193093772");
  }
}
