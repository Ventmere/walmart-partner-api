use chrono::{Duration, Utc};
use clap::ArgMatches;
use serde_json;
use walmart_partner_api::order::*;
use walmart_partner_api::Client;

pub fn list(client: &Client, status: Option<&str>) {
  let mut query: QueryParams = Default::default();
  let start_date = (Utc::now() - Duration::days(7)).date().and_hms(0, 0, 0);
  query.createdStartDate = Some(start_date);
  query.createdEndDate = Some(Utc::now());
  query.status = status.or(Some("Created")).map(|s| s.to_owned());
  let res = client.get_all_orders(&query).unwrap();
  println!("{:#?}", res);
}

pub fn list_wfs(client: &Client, status: Option<&str>) {
  let mut query: WFSQueryParams = Default::default();
  let start_date = (Utc::now() - Duration::days(7));
  query.createdStartDate = Some(start_date);
  query.createdEndDate = Some(Utc::now());
  // query.status = status.or(Some("Created")).map(|s| s.to_owned());
  // query.status = status.or(Some("Acknowledged")).map(|s| s.to_owned());
  let res = client.get_all_wfs_orders(&query).unwrap();
  println!("{:#?}", res);
}

pub fn list_released(client: &Client) {
  let query: ReleasedQueryParams = Default::default();
  let res = client.get_all_released_orders(&query).unwrap();
  println!("{:#?}", res);
}

pub fn list_status(client: &Client, status: &str) {
  let mut query: QueryParams = Default::default();
  let start_date = (Utc::now() - Duration::days(30)).date().and_hms(0, 0, 0);
  query.createdStartDate = Some(start_date);
  query.status = Some(status.to_string());
  let res = client.get_all_orders(&query).unwrap();
  println!("{:#?}", res);
}

pub fn dump(client: &Client) {
  let mut query: QueryParams = Default::default();
  query.limit = Some(200);
  let start_date = (Utc::now() - Duration::days(100)).date().and_hms(0, 0, 0);
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
    ::std::thread::sleep(::std::time::Duration::from_secs(1));
  }

  println!("{}", serde_json::to_string_pretty(&elements).unwrap());
}

pub fn get(client: &Client, id: &str) {
  let res = client.get_order(id).unwrap();
  println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

pub fn ship(client: &Client, m: &ArgMatches) {
  let params = ShipParams {
    lineNumber: m
      .value_of("line_number")
      .map(ToString::to_string)
      .unwrap_or_else(|| "1".to_string()),
    shipDateTime: Utc::now(),
    carrierName: m.value_of("carrier_name").map(ToString::to_string),
    methodCode: m
      .value_of("method")
      .map(ToString::to_string)
      .unwrap_or_else(|| "Standard".to_string()),
    trackingNumber: m
      .value_of("tracking_number")
      .map(ToString::to_string)
      .unwrap(),
    trackingURL: m
      .value_of("tracking_url")
      .map(ToString::to_string)
      .unwrap_or_default(),
    otherCarrier: m.value_of("other_carrier").map(ToString::to_string),
    unitOfMeasurement: m.value_of("unit_of_measurement").map(ToString::to_string),
    amount: m.value_of("amount").map(ToString::to_string),
    shipFromCountry: m
      .value_of("shipFromCountry")
      .map(ToString::to_string)
      .unwrap(),
  };
  let res = client
    .ship_order_line(m.value_of("ORDER_ID").unwrap(), &params)
    .unwrap();
  println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

pub fn ack(client: &Client, po_id: &str) {
  let res = client.ack_order(po_id).unwrap();
  println!("{}", serde_json::to_string_pretty(&res).unwrap());
}
