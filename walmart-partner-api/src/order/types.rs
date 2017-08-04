#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CurrencyAmount {
  pub currency: String,
  pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PostalAddress {
  pub name: String,
  pub address1: String,
  pub address2: Option<String>,
  pub city: String,
  pub state: String,
  pub postalCode: String,
  pub country: String,
  pub addressType: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ShippingInformation {
  pub phone: String,
  pub estimatedDeliveryDate: i64,
  pub estimatedShipDate: i64,
  pub methodCode: String,
  pub postalAddress: PostalAddress,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OrderLineItem {
  pub productName: String,
  pub sku: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Tax {
  pub taxName: String,
  pub taxAmount: CurrencyAmount,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OrderLineChargeItem {
  pub chargeType: String,
  pub chargeName: String,
  pub chargeAmount: CurrencyAmount,
  pub tax: Option<Tax>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Quantity {
  pub unitOfMeasurement: String,
  pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OrderLineCharges {
  pub charge: Vec<OrderLineChargeItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OrderLineTrackingInfoCarrier {
  pub otherCarrier: Option<String>,
  pub carrier: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OrderLineTrackingInfo {
  pub shipDateTime: i64,
  pub carrierName: OrderLineTrackingInfoCarrier,
  pub methodCode: String,
  pub trackingNumber: String,
  pub trackingURL: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OrderLineStatus {
  pub status: String, 
  pub statusQuantity: Quantity,
  // pub cancellationReason: Option<?>,
  pub trackingInfo: Option<OrderLineTrackingInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OrderLineStatuss {
  pub orderLineStatus: Vec<OrderLineStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OrderLine {
  pub lineNumber: String,
  pub item: OrderLineItem,
  pub charges: OrderLineCharges,
  pub orderLineQuantity: Quantity,
  pub statusDate: i64,
  pub orderLineStatuses: OrderLineStatuss,
  // pub refund: Option<?>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OrderLines {
  pub orderLine: Vec<OrderLine>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Order {
  pub purchaseOrderId: String,
  pub customerOrderId: String,
  pub customerEmailId: String,
  pub orderDate: i64,
  pub shippingInfo: ShippingInformation,
  pub orderLines: OrderLines,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    use serde_json;

    let json = r##"
      [
        {
          "purchaseOrderId": "11",
          "customerOrderId": "12",
          "customerEmailId": "1@relay.walmart.com",
          "orderDate": 1501903867000,
          "shippingInfo": {
            "phone": "9176070000",
            "estimatedDeliveryDate": 1502863200000,
            "estimatedShipDate": 1502258400000,
            "methodCode": "Standard",
            "postalAddress": {
              "name": "Foo Bar",
              "address1": "7777 Madisoner Ct",
              "address2": null,
              "city": "Fake",
              "state": "VA",
              "postalCode": "78787",
              "country": "USA",
              "addressType": "RESIDENTIAL"
            }
          },
          "orderLines": {
            "orderLine": [
              {
                "lineNumber": "1",
                "item": {
                  "productName": "Edifier H850 Over-the-ear Pro Headphones",
                  "sku": "edifier-h850"
                },
                "charges": {
                  "charge": [
                    {
                      "chargeType": "PRODUCT",
                      "chargeName": "ItemPrice",
                      "chargeAmount": {
                        "currency": "USD",
                        "amount": 39.99
                      },
                      "tax": null
                    }
                  ]
                },
                "orderLineQuantity": {
                  "unitOfMeasurement": "EACH",
                  "amount": "1"
                },
                "statusDate": 1502114839000,
                "orderLineStatuses": {
                  "orderLineStatus": [
                    {
                      "status": "Shipped",
                      "statusQuantity": {
                        "unitOfMeasurement": "EACH",
                        "amount": "1"
                      },
                      "cancellationReason": null,
                      "trackingInfo": {
                        "shipDateTime": 1502089301000,
                        "carrierName": {
                          "otherCarrier": "OtherCarrier",
                          "carrier": null
                        },
                        "methodCode": "Standard",
                        "trackingNumber": "TBA387619000000",
                        "trackingURL": "https://edifier-usa.myshopify.com/admin/orders/5994717066"
                      }
                    }
                  ]
                },
                "refund": null
              }
            ]
          }
        },
        {
          "purchaseOrderId": "21",
          "customerOrderId": "22",
          "customerEmailId": "2@relay.walmart.com",
          "orderDate": 1491380201000,
          "shippingInfo": {
            "phone": "6026900000",
            "estimatedDeliveryDate": 1492236000000,
            "estimatedShipDate": 1491631200000,
            "methodCode": "Standard",
            "postalAddress": {
              "name": "Joe Da",
              "address1": "1234 Vard St.  Apt 4444",
              "address2": null,
              "city": "Fake",
              "state": "AZ",
              "postalCode": "77777",
              "country": "USA",
              "addressType": "RESIDENTIAL"
            }
          },
          "orderLines": {
            "orderLine": [
              {
                "lineNumber": "1",
                "item": {
                  "productName": "Edifier R1280T Powered Bookshelf Speakers Studio Monitors",
                  "sku": "EDIFIERr1280t"
                },
                "charges": {
                  "charge": [
                    {
                      "chargeType": "PRODUCT",
                      "chargeName": "ItemPrice",
                      "chargeAmount": {
                        "currency": "USD",
                        "amount": 0.0
                      },
                      "tax": null
                    }
                  ]
                },
                "orderLineQuantity": {
                  "unitOfMeasurement": "EACH",
                  "amount": "1"
                },
                "statusDate": 1491839507000,
                "orderLineStatuses": {
                  "orderLineStatus": [
                    {
                      "status": "Cancelled",
                      "statusQuantity": {
                        "unitOfMeasurement": "EACH",
                        "amount": "1"
                      },
                      "cancellationReason": null,
                      "trackingInfo": null
                    }
                  ]
                },
                "refund": null
              }
            ]
          }
        }
      ]
    "##;

    let orders: Vec<Order> = serde_json::from_str(&json).unwrap();
    assert_eq!(orders[0].orderLines.orderLine[0].lineNumber, "1");
    assert_eq!(orders[0].orderLines.orderLine[0].item.sku, "edifier-h850");
  }
}