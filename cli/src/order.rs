use chrono::{Utc, Duration};
use serde_json;
use walmart_partner_api::Client;
use walmart_partner_api::order::*;

pub fn list(client: &Client, status: Option<&str>) {
  let mut query: QueryParams = Default::default();
  let start_date = (Utc::now() - Duration::days(7)).date().and_hms(0, 0, 0);
  query.createdStartDate = Some(start_date);
  // query.createdEndDate = Some(Utc::now());
  query.status = status.or(Some("Created")).map(|s| s.to_owned());
  let res = client.get_all_orders(&query).unwrap();
  println!("{:#?}", res);
}

pub fn dump(client: &Client) {
  let mut query: QueryParams = Default::default();
  query.limit = Some(200);
  let start_date = (Utc::now() - Duration::days(365)).date().and_hms(0, 0, 0);
  query.createdStartDate = Some(start_date);

  let res = client.get_all_orders(&query).unwrap();

  let mut next_cursor = res.get_next_cursor().map(str::to_string);
  let mut elements = res.elements;
  while let Some(cursor) = next_cursor {
    let mut res = client.get_all_orders_by_next_cursor(&cursor).unwrap();
    next_cursor = res.get_next_cursor().map(str::to_string);
    // println!(
    //   "n = {:?}, next_cursur = {:?}",
    //   res.get_total_count(),
    //   next_cursor
    // );
    elements.append(&mut res.elements);
    ::std::thread::sleep_ms(1000);
  }

  println!("{}", serde_json::to_string_pretty(&elements).unwrap());
}

pub fn get(client: &Client, id: &str) {
  let res = client.get_order(id).unwrap();
  println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

pub fn ship(client: &Client) {
  use chrono::Utc;
  let params = ShipParams {
    lineNumber: "1".to_string(),
    shipDateTime: Utc::now(),
    carrierName: "FedEx".to_string(),
    methodCode: "Standard".to_owned(),
    trackingNumber: "714510651980".to_string(),
    trackingURL: "".to_string(),
  };
  let res = client.ship_order_line("2581004628475", &params).unwrap();
  println!("{}", serde_json::to_string_pretty(&res).unwrap());
}