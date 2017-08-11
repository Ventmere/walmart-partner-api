extern crate walmart_partner_api;
extern crate dotenv;
#[macro_use] extern crate clap;

use std::env;
use walmart_partner_api::Client;

mod upload;

fn main() {
  dotenv::dotenv().ok();
  let matches = clap_app!(cli =>
    (version: "0.1")
    (about: "Walmart CLI")
    (@subcommand upload =>
      (about: "uploads feed file to walmart")
      (@arg feed_type: -t --type +required +takes_value "Sets the feed type")
      (@arg INPUT: +required "Sets the feed file to upload")
    )
  ).get_matches();

  let client = Client::new(
    &env::var("WALMART_CONSUMER_ID").unwrap(),
    &env::var("WALMART_PRIVATE_KEY").unwrap()
  ).unwrap();

  if let Some(matches) = matches.subcommand_matches("upload") {
    upload::upload_feed(&client, 
      matches.value_of("feed_type").unwrap(),
      matches.value_of("INPUT").unwrap()
    );
  }
}