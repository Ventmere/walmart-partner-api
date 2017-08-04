extern crate walmart_partner_api;
extern crate dotenv;
extern crate chrono;
#[macro_use] extern crate clap;

use std::env;
use walmart_partner_api::Client;

mod feed;
mod order;

fn main() {
  dotenv::dotenv().ok();
  let matches = clap_app!(cli =>
    (version: "0.1")
    (about: "Walmart CLI")
    (@subcommand feed =>
      (about: "Feed API")
      (@subcommand upload =>
        (about: "upload feed")
        (@arg feed_type: -t --type +required +takes_value "Sets the feed type")
        (@arg INPUT: +required "Sets the feed file to upload")
      )
      (@subcommand status =>
        (about: "list all feed statues")
      )
      (@subcommand inspect =>
        (about: "inspect uploaded feed")
        (@arg FEED_ID: +required "Sets the feed id to inspect")
      )
    )
    (@subcommand order =>
      (about: "Order API")
      (@subcommand list =>
        (about: "Get orders created in last 24 hours")
        (@arg STATUS: "Sets the order status (default: Created)")        
      )
    )
  ).get_matches();

  let client = Client::new(
    &env::var("WALMART_CONSUMER_ID").unwrap(),
    &env::var("WALMART_PRIVATE_KEY").unwrap()
  ).unwrap();
  
  match matches.subcommand() {
    ("feed", Some(matches)) => {
      match matches.subcommand() {
        ("upload", Some(matches)) => {
          feed::upload(&client, 
            matches.value_of("feed_type").unwrap(),
            matches.value_of("INPUT").unwrap()
          );
        },
        ("status", _) => {
          feed::status(&client);
        },
        ("inspect", Some(matches)) => {
          feed::inspect(&client,
            matches.value_of("FEED_ID").unwrap()
          );
        },
        _ => {},
      }
    },
    ("order", Some(matches)) => {
      match matches.subcommand() {
        ("list", m) => {
          order::list(&client, m.and_then(|m| m.value_of("STATUS")));
        },
        _ => {},
      }
    },
    _ => {},
  }
}