use chrono::{Utc, Duration};
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
