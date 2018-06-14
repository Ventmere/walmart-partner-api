extern crate chrono;
extern crate dotenv;
extern crate walmart_partner_api;
#[macro_use]
extern crate clap;
extern crate serde_json;

use std::env;
use walmart_partner_api::{Client, WalmartMarketplace};

mod feed;
mod order;
mod report;

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
      (@subcommand dump =>
        (about: "dump top 200 orders in last 365 days")
      )
      (@subcommand get =>
        (about: "get order")
        (@arg ORDER_ID: +required "Sets the order id")
      )
      (@subcommand ship =>
        (about: "ship order")
      )
    )
    (@subcommand report =>
      (about: "Report API")
      (@subcommand get =>
        (about: "get report")
        (@arg report_type: -t --type +required +takes_value "Sets the report type")
      )
    )
  ).get_matches();

  let client = Client::new(
    match env::var("WALMART_MARKETPLACE").unwrap().as_ref() {
      "USA" => WalmartMarketplace::USA,
      "Canada" => WalmartMarketplace::Canada,
      _ => unreachable!(),
    },
    &env::var("WALMART_CHANNEL_TYPE").unwrap(),
    &env::var("WALMART_CONSUMER_ID").unwrap(),
    &env::var("WALMART_PRIVATE_KEY").unwrap(),
  ).unwrap();

  match matches.subcommand() {
    ("feed", Some(matches)) => match matches.subcommand() {
      ("upload", Some(matches)) => {
        feed::upload(
          &client,
          matches.value_of("feed_type").unwrap(),
          matches.value_of("INPUT").unwrap(),
        );
      }
      ("status", _) => {
        feed::status(&client);
      }
      ("inspect", Some(matches)) => {
        feed::inspect(&client, matches.value_of("FEED_ID").unwrap());
      }
      _ => {}
    },
    ("order", Some(matches)) => match matches.subcommand() {
      ("list", m) => {
        order::list(&client, m.and_then(|m| m.value_of("STATUS")));
      }
      ("get", Some(m)) => {
        order::get(&client, m.value_of("ORDER_ID").unwrap());
      }
      ("dump", _) => {
        order::dump(&client);
      }
      ("ship", _) => {
        order::ship(&client);
      }
      _ => {}
    },
    ("report", Some(matches)) => match matches.subcommand() {
      ("get", Some(matches)) => {
        report::get(&client, matches.value_of("report_type").unwrap());
      }
      _ => {}
    },
    _ => {}
  }
}
