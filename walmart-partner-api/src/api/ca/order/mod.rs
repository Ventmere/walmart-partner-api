use chrono::{DateTime, Utc};
use serde::Serialize;
use xml_builder::XMLElement;

pub use types::*;

use crate::api::ca::Client;
use crate::client::Method;
use crate::{WalmartResult, XmlSer};

mod types;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllReleasedOrdersQuery {
  pub limit: Option<i32>,
  pub created_start_date: DateTime<Utc>,
  pub created_end_date: Option<DateTime<Utc>>,
  pub product_info: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllOrdersQuery {
  pub created_start_date: DateTime<Utc>,
  pub sku: Option<String>,
  pub customer_order_id: Option<String>,
  pub purchase_order_id: Option<String>,
  pub status: Option<String>,
  pub created_end_date: Option<DateTime<Utc>>,
  pub from_expected_ship_date: Option<DateTime<Utc>>,
  pub to_expected_ship_date: Option<DateTime<Utc>>,
  pub limit: Option<i32>,
  pub product_info: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrderQuery {
  pub product_info: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ShipOrderLines {
  pub order_lines: Vec<ShipOrderLine>,
}

#[derive(Debug, Clone)]
pub struct ShipOrderLine {
  pub line_number: String,
  pub ship_from_country: String,
  /// If not provided the default values are used
  pub status_quantity: Option<ShipOrderLineStatusQuantity>,
  pub asn: Option<ShipOrderLineAsn>,
  pub tracking_info: OrderLineTrackingInfo,
}

impl Client {
  pub async fn get_all_orders(&self, query: GetAllOrdersQuery) -> WalmartResult<OrderList> {
    let qs = serde_urlencoded::to_string(query)?;
    let req = self.req_xml(Method::GET, "/v3/ca/orders", qs)?;
    self.send(req).await?.res_xml().await
  }

  pub async fn get_all_released_orders(
    &self,
    query: GetAllReleasedOrdersQuery,
  ) -> WalmartResult<OrderList> {
    let qs = serde_urlencoded::to_string(query)?;
    let req = self.req_xml(Method::GET, "/v3/ca/orders/released", qs)?;
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
      &format!("/v3/ca/orders/{}", purchase_order_id.as_ref()),
      qs,
    )?;
    self.send(req).await?.res_xml().await
  }

  pub async fn ack_order(&self, purchase_order_id: impl AsRef<str>) -> WalmartResult<Order> {
    let req = self.req_xml(
      Method::POST,
      &format!("/v3/ca/orders/{}/acknowledge", purchase_order_id.as_ref()),
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
        &format!("/v3/ca/orders/{}/shipping", purchase_order_id.as_ref()),
        (),
      )?
      .body_xml(input)?;

    self.send(req).await?.res_xml().await
  }
}

impl XmlSer for ShipOrderLines {
  fn to_xml(&self) -> WalmartResult<XMLElement> {
    let mut order_shipment = XMLElement::new("orderShipment");
    order_shipment.add_attribute("xmlns:ns2", "http://walmart.com/mp/v3/orders");
    order_shipment.add_attribute("xmlns:ns3", "http://walmart.com/");

    let mut order_lines = XMLElement::new("orderLines");

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

    let mut ship_from_country = XMLElement::new("shipFromCountry");
    ship_from_country.add_text(self.ship_from_country.clone())?;
    order_line.add_child(ship_from_country)?;

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
    let client = crate::test_util::get_client_ca();

    let _m = mock("GET", "/v3/ca/orders")
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

    assert_eq!(got.elements.order.len(), 2);
    let order = got.elements.order[0].clone();
    assert_eq!(order.purchase_order_id, "2575263094491");
    let date = DateTime::<Utc>::from_str("2016-05-18T16:53:14.000Z").unwrap();
    assert_eq!(order.order_date, date);
    assert_eq!(order.shipping_info.method_code, "Standard");
    assert_eq!(order.order_lines.order_line.len(), 1);
    let line = order.order_lines.order_line[0].clone();
    assert_eq!(line.line_number, 4.to_string());
    assert_eq!(line.charges.charge.len(), 1);
  }

  #[tokio::test]
  async fn test_get_all_released_orders() {
    let client = crate::test_util::get_client_ca();
    let body = r#"
    <?xml version="1.0" encoding="UTF-8"?>
<list xmlns:ns3="http://walmart.com/mp/v3/orders" xmlns:ns2="http://walmart.com/mp/orders" xmlns:ns4="http://walmart.com/">
    <meta>
        <totalCount>8</totalCount>
        <limit>1</limit>
        <nextCursor>?limit=1&amp;hasMoreElements=true&amp;soIndex=8&amp;poIndex=1577014034085&amp;sellerId=801&amp;status=Created&amp;createdStartDate=2016-11-01&amp;createdEndDate=2016-12-12T22:50:46.014Z</nextCursor>
    </meta>
    <elements>
        <order>
            <purchaseOrderId>1577014034085</purchaseOrderId>
            <customerOrderId>5851600737546</customerOrderId>
            <customerEmailId>mgr@walmartlabs.com</customerEmailId>
            <orderDate>2016-11-09T00:31:31.000Z</orderDate>
            <shippingInfo>
                <phone>6502248603</phone>
                <estimatedDeliveryDate>2016-11-22T07:00:00.000Z</estimatedDeliveryDate>
                <estimatedShipDate>2016-11-09T07:00:00.000Z</estimatedShipDate>
                <methodCode>Value</methodCode>
                <postalAddress>
                    <name>Madhukara
                        PGOMS</name>
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
                    <lineNumber>1</lineNumber>
                    <item>
                        <productName>Ozark
                            Trail 4-Person Dome Tent</productName>
                        <sku>NJ_WITHOUT_RCA_003</sku>
                    </item>
                    <charges>
                        <charge>
                            <chargeType>PRODUCT</chargeType>
                            <chargeName>ItemPrice</chargeName>
                            <chargeAmount>
                                <currency>USD</currency>
                                <amount>21.99</amount>
                            </chargeAmount>
                            <tax>
                                <taxName>Tax1</taxName>
                                <taxAmount>
                                    <currency>USD</currency>
                                    <amount>1.92</amount>
                                </taxAmount>
                            </tax>
                        </charge>
                    </charges>
                    <orderLineQuantity>
                        <unitOfMeasurement>EACH</unitOfMeasurement>
                        <amount>1</amount>
                    </orderLineQuantity>
                    <statusDate>2016-11-09T00:32:44.000Z</statusDate>
                    <orderLineStatuses>
                        <orderLineStatus>
                            <status>Created</status>
                            <statusQuantity>
                                <unitOfMeasurement>EACH</unitOfMeasurement>
                                <amount>1</amount>
                            </statusQuantity>
                        </orderLineStatus>
                    </orderLineStatuses>
                </orderLine>
                <orderLine>
                    <lineNumber>2</lineNumber>
                    <item>
                        <productName>Lacoste
                            Sienna Stainless Steel Womens Fashion Watch White Strap 2000855</productName>
                        <sku>2000855-custom-1a</sku>
                    </item>
                    <charges>
                        <charge>
                            <chargeType>PRODUCT</chargeType>
                            <chargeName>ItemPrice</chargeName>
                            <chargeAmount>
                                <currency>USD</currency>
                                <amount>75.00</amount>
                            </chargeAmount>
                            <tax>
                                <taxName>Tax1</taxName>
                                <taxAmount>
                                    <currency>USD</currency>
                                    <amount>6.57</amount>
                                </taxAmount>
                            </tax>
                        </charge>
                    </charges>
                    <orderLineQuantity>
                        <unitOfMeasurement>EACH</unitOfMeasurement>
                        <amount>1</amount>
                    </orderLineQuantity>
                    <statusDate>2016-11-09T00:32:44.000Z</statusDate>
                    <orderLineStatuses>
                        <orderLineStatus>
                            <status>Created</status>
                            <statusQuantity>
                                <unitOfMeasurement>EACH</unitOfMeasurement>
                                <amount>1</amount>
                            </statusQuantity>
                        </orderLineStatus>
                    </orderLineStatuses>
                </orderLine>
            </orderLines>
        </order>
    </elements>
</list>
    "#;

    let _m = mock("GET", "/v3/ca/orders/released")
      .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
        "limit".into(),
        "10".into(),
      )]))
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let got = client
      .get_all_released_orders(GetAllReleasedOrdersQuery {
        limit: Some(10),
        ..Default::default()
      })
      .await
      .unwrap();
    assert_eq!(got.meta.total_count, Some(8));
    assert_eq!(got.elements.order.len(), 1);
  }

  #[tokio::test]
  async fn test_get_order() {
    let client = crate::test_util::get_client_ca();
    let body = r#"
    <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ns3:order xmlns:ns2="http://walmart.com/mp/orders" xmlns:ns3="http://walmart.com/mp/v3/orders" xmlns:ns4="com.walmart.services.common.model.money" xmlns:ns5="com.walmart.services.common.model.name" xmlns:ns6="com.walmart.services.common.model.address" xmlns:ns7="com.walmart.services.common.model.address.validation" xmlns:ns8="http://walmart.com/">
   <ns3:purchaseOrderId>1796330120075</ns3:purchaseOrderId>
   <ns3:customerOrderId>1568964842632</ns3:customerOrderId>
   <ns3:customerEmailId>3944889D125D4AA599DCEB84056C2183@relay.walmart.com</ns3:customerEmailId>
   <ns3:orderDate>2019-09-19T13:03:55.000+05:30</ns3:orderDate>
   <ns3:shippingInfo>
      <ns3:phone>5106793150</ns3:phone>
      <ns3:estimatedDeliveryDate>2019-10-04T00:30:00.000+05:30</ns3:estimatedDeliveryDate>
      <ns3:estimatedShipDate>2019-10-02T10:00:00.000+05:30</ns3:estimatedShipDate>
      <ns3:methodCode>OneDay</ns3:methodCode>
      <ns3:postalAddress>
         <ns3:name>Shai Kodela</ns3:name>
         <ns3:address1>640 West California Avenue</ns3:address1>
         <ns3:address2>asdasd</ns3:address2>
         <ns3:state>CA</ns3:state>
         <ns3:city>Sunnyvale</ns3:city>
         <ns3:postalCode>94043</ns3:postalCode>
         <ns3:country>USA</ns3:country>
         <ns3:addressType>RESIDENTIAL</ns3:addressType>
      </ns3:postalAddress>
   </ns3:shippingInfo>
   <ns3:orderLines>
      <ns3:orderLine>
         <ns3:lineNumber>1</ns3:lineNumber>
         <ns3:item>
            <ns3:sku>TEST-02</ns3:sku>
            <ns3:productName>Item 1</ns3:productName>
         </ns3:item>
         <ns3:charges>
            <ns3:charge>
               <ns3:chargeType>PRODUCT</ns3:chargeType>
               <ns3:chargeName>ItemPrice</ns3:chargeName>
               <ns3:chargeAmount>
                  <ns3:currency>USD</ns3:currency>
                  <ns3:amount>0.00</ns3:amount>
               </ns3:chargeAmount>
            </ns3:charge>
            <ns3:charge>
               <ns3:chargeType>SHIPPING</ns3:chargeType>
               <ns3:chargeName>Shipping</ns3:chargeName>
               <ns3:chargeAmount>
                  <ns3:currency>USD</ns3:currency>
                  <ns3:amount>0.00</ns3:amount>
               </ns3:chargeAmount>
            </ns3:charge>
         </ns3:charges>
         <ns3:orderLineQuantity>
            <ns3:unitOfMeasurement>EACH</ns3:unitOfMeasurement>
            <ns3:amount>1</ns3:amount>
         </ns3:orderLineQuantity>
         <ns3:statusDate>2019-10-01T11:07:00.727+05:30</ns3:statusDate>
         <ns3:orderLineStatuses>
            <ns3:orderLineStatus>
               <ns3:status>Created</ns3:status>
               <ns3:statusQuantity>
                  <ns3:unitOfMeasurement>EACH</ns3:unitOfMeasurement>
                  <ns3:amount>1</ns3:amount>
               </ns3:statusQuantity>
            </ns3:orderLineStatus>
         </ns3:orderLineStatuses>
         <ns3:originalCarrierMethod>67</ns3:originalCarrierMethod>
         <ns3:fulfillment>
            <ns3:fulfillmentOption>S2H</ns3:fulfillmentOption>
            <ns3:shipMethod>RUSH</ns3:shipMethod>
            <ns3:pickUpDateTime>2019-09-30T13:03:55.000+05:30</ns3:pickUpDateTime>
            <ns3:pickUpBy>Raj Accept Kumar</ns3:pickUpBy>
            <ns3:shippingProgramType>TWO_DAY</ns3:shippingProgramType>
         </ns3:fulfillment>

      </ns3:orderLine>
   </ns3:orderLines>
   <ns3:shipNode>
      <ns3:type>3PLFulfilled</ns3:type>
   </ns3:shipNode>
</ns3:order>
    "#;

    let _m = mock("GET", "/v3/ca/orders/1796330120075")
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let got = client
      .get_order("1796330120075", Default::default())
      .await
      .unwrap();
    assert_eq!(got.purchase_order_id, "1796330120075");
    assert_eq!(got.customer_order_id, "1568964842632");
    assert_eq!(got.order_lines.order_line.len(), 1);
  }

  #[tokio::test]
  pub async fn test_ack_order() {
    let client = crate::test_util::get_client_ca();
    let body = r#"
    <?xml version="1.0" encoding="UTF-8" standalone="yes" ?>
<ns3:order
  xmlns:ns3="http://walmart.com/mp/orders">
    <ns3:purchaseOrderId>2575193093772</ns3:purchaseOrderId>
    <ns3:customerOrderId>4021603841173</ns3:customerOrderId>
    <ns3:customerEmailId>mgr@walmartlabs.com</ns3:customerEmailId>
    <ns3:orderDate>2016-05-11T22:55:28.000Z</ns3:orderDate>
    <ns3:shippingInfo>
        <ns3:phone>6502248603</ns3:phone>
        <ns3:estimatedDeliveryDate>2016-05-20T17:00:00.000Z</ns3:estimatedDeliveryDate>
        <ns3:estimatedShipDate>2016-05-16T17:00:00.000Z</ns3:estimatedShipDate>
        <ns3:methodCode>Standard</ns3:methodCode>
        <ns3:postalAddress>
            <ns3:name>Joe Doe PGOMS</ns3:name>
            <ns3:address1>860 W Cal Ave</ns3:address1>
            <ns3:address2>Seat # 860C.2.176</ns3:address2>
            <ns3:city>Sunnyvale</ns3:city>
            <ns3:state>CA</ns3:state>
            <ns3:postalCode>94086</ns3:postalCode>
            <ns3:country>USA</ns3:country>
            <ns3:addressType>RESIDENTIAL</ns3:addressType>
        </ns3:postalAddress>
    </ns3:shippingInfo>
    <ns3:orderLines>
        <ns3:orderLine>
            <ns3:lineNumber>2</ns3:lineNumber>
            <ns3:item>
                <ns3:productName>Garmin Refurbished nuvi 2595LMT 5 GPS w Lifetime Maps and Traffic</ns3:productName>
                <ns3:sku>GRMN100201</ns3:sku>
            </ns3:item>
            <ns3:charges>
                <ns3:charge>
                    <ns3:chargeType>PRODUCT</ns3:chargeType>
                    <ns3:chargeName>ItemPrice</ns3:chargeName>
                    <ns3:chargeAmount>
                        <ns3:currency>USD</ns3:currency>
                        <ns3:amount>124.98</ns3:amount>
                    </ns3:chargeAmount>
                    <ns3:tax>
                        <ns3:taxName>Tax1</ns3:taxName>
                        <ns3:taxAmount>
                            <ns3:currency>USD</ns3:currency>
                            <ns3:amount>10.93</ns3:amount>
                        </ns3:taxAmount>
                    </ns3:tax>
                </ns3:charge>
            </ns3:charges>
            <ns3:orderLineQuantity>
                <ns3:unitOfMeasurement>EACH</ns3:unitOfMeasurement>
                <ns3:amount>1</ns3:amount>
            </ns3:orderLineQuantity>
            <ns3:statusDate>2016-06-03T23:32:51.000Z</ns3:statusDate>
            <ns3:orderLineStatuses>
                <ns3:orderLineStatus>
                    <ns3:status>Acknowledged</ns3:status>
                    <ns3:statusQuantity>
                        <ns3:unitOfMeasurement>EACH</ns3:unitOfMeasurement>
                        <ns3:amount>1</ns3:amount>
                    </ns3:statusQuantity>
                </ns3:orderLineStatus>
            </ns3:orderLineStatuses>
        </ns3:orderLine>
    </ns3:orderLines>
</ns3:order>
    "#;

    let _m = mock("POST", "/v3/ca/orders/2575193093772/acknowledge")
      .with_status(200)
      .with_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let got = client.ack_order("2575193093772").await.unwrap();
    assert_eq!(got.purchase_order_id, "2575193093772");
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
        order_lines: vec![ShipOrderLine {
          line_number: "2".to_string(),
          ship_from_country: "CA".to_string(),
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
  xmlns:ns2="http://walmart.com/mp/v3/orders"
  xmlns:ns3="http://walmart.com/">
    <orderLines>
        <orderLine>
            <lineNumber>2</lineNumber>
            <shipFromCountry>CA</shipFromCountry>
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
    let client = crate::test_util::get_client_ca();
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
            <shipFromCountry>CA</shipFromCountry>
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

    let _m = mock("POST", "/v3/ca/orders/2575193093772/shipping")
      .with_status(200)
      .with_header("content-type", "application/xml")
      .match_header("content-type", "application/xml")
      .with_body(body)
      .create();

    let input = ShipOrderLines {
      order_lines: vec![ShipOrderLine {
        line_number: "2".to_string(),
        ship_from_country: "CA".to_string(),
        status_quantity: None,
        asn: None,
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
    };

    let got = client
      .ship_order_lines("2575193093772", input)
      .await
      .unwrap();
    assert_eq!(got.purchase_order_id, "2575193093772");
  }
}
