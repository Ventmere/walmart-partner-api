use std::fs::File;
use walmart_partner_api::Client;
use walmart_partner_api::feed::*;

pub fn upload_feed(client: &Client, feed_type: &str, path: &str) {
  println!("feed file = {}", path);
  let f = File::open(path).unwrap();
  let ack = client.bulk_upload(feed_type, f).unwrap();
  println!("ack = {:#?}", ack);
}