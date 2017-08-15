use std::fs::File;
use walmart_partner_api::Client;

pub fn upload(client: &Client, feed_type: &str, path: &str) {
  let f = File::open(path).unwrap();
  let ack = client.bulk_upload(feed_type, f).unwrap();
  println!("{:#?}", ack);
}

pub fn status(client: &Client) {
  let status = client.get_all_feed_statuses(&Default::default()).unwrap();
  println!("{:#?}", status);
}

pub fn inspect(client: &Client, id: &str) {
  let status = client.get_feed_and_item_status(id, &Default::default()).unwrap();
  println!("{:#?}", status);
}