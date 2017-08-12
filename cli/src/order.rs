use chrono::{Utc, Duration};
use walmart_partner_api::Client;
use walmart_partner_api::order::*;

pub fn list(client: &Client, status: Option<&str>) {
  let mut query: QueryParams = Default::default();
  query.createdStartDate = Some(Utc::now() - Duration::hours(24));
  // query.createdEndDate = Some(Utc::now());
  query.status = status.or(Some("Created")).map(|s| s.to_owned());
  let res = client.get_all_orders(&query).unwrap();
  println!("{:#?}", res);
}
