extern crate chrono;
extern crate dotenv;
extern crate walmart_partner_api;
#[macro_use]
extern crate clap;
extern crate serde_json;

use std::env;
use walmart_partner_api::{Client, WalmartCredential, WalmartMarketplace};

mod feed;
mod inventory;
mod item;
mod order;
mod report;

fn main() {
  let matches = clap_app!(cli =>
    (version: "0.1")
    (about: "Walmart CLI")
    (@arg ENV: -e --env +takes_value "Sets a custom env file")
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
      (@subcommand list_wfs =>
        (about: "Get WFS orders created in last 24 hours")
        (@arg STATUS: "Sets the order status (default: Created)")
      )
      (@subcommand list_released => 
      )
      (@subcommand list_status =>
        (about: "Get orders with status")
        (@arg STATUS: "Sets the order status")        
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
        (@arg ORDER_ID: +required "Sets the order id")
        (@arg line_number: -n --line_number +takes_value "Sets the line number, default 1")
        (@arg method: -m --method_code +takes_value "Sets the method code, default 'Standard'")
        (@arg other_carrier: -o --other_carrier +takes_value "Sets the otherCarrier")
        (@arg unit_of_measurement: -u --unit_of_measurement +takes_value "Sets the unitOfMeasurement")
        (@arg amount: -a --amount +takes_value "Sets the amount")
        (@arg carrier_name: -c --carrier_name +takes_value "Sets the carrier name")
        (@arg tracking_number: -t --tracking_number +takes_value "Sets the tracking number")
        (@arg tracking_url: -r --tracking_url +takes_value "Sets the tracking url")
      )
      (@subcommand ack =>
        (about: "ack order")
        (@arg PO_ID: +required "Sets the po id")
      )
    )
    (@subcommand report =>
      (about: "Report API")
      (@subcommand get =>
        (about: "get report")
        (@arg report_type: -t --type +required +takes_value "Sets the report type")
      )
      (@subcommand get_raw =>
        (about: "get report to file")
        (@arg report_type: -t --type +required +takes_value "Sets the report type")
        (@arg out: -o --out +required +takes_value "Sets the output path")
      )
    )
    (@subcommand item =>
      (about: "Item API")
      (@subcommand dump =>
        (about: "dump items")
      )
      (@subcommand dump_us =>
        (about: "dump us items")
      )
    )
    (@subcommand inventory =>
      (about: "Inventory API")
      (@subcommand set =>
        (about: "set sku inventory")
        (@arg sku: -s --sku +required +takes_value "SKU")
        (@arg quantity: -q --quantity +required +takes_value "Quantity")
        (@arg lagtime: -l --lagtime +required +takes_value "Fulfillment Lag Time")
      )
      (@subcommand get =>
        (about: "get sku inventory")
        (@arg sku: -s --sku +required +takes_value "SKU")
      )
    )
  )
  .get_matches();

  match matches.value_of("ENV") {
    Some(path) => {
      dotenv::from_path(::std::path::Path::new(path)).unwrap();
    }
    None => {
      dotenv::dotenv().unwrap();
    }
  }

  env_logger::init();

  let client = Client::new(
    match env::var("WALMART_MARKETPLACE").unwrap().as_ref() {
      "USA" => WalmartMarketplace::USA,
      "Canada" => WalmartMarketplace::Canada,
      _ => unreachable!(),
    },
    if env::var("WALMART_CLIENT_ID").ok().is_some() {
      WalmartCredential::TokenApi {
        client_id: env::var("WALMART_CLIENT_ID").unwrap(),
        client_secret: env::var("WALMART_CLIENT_SECRET").unwrap(),
      }
    } else {
      WalmartCredential::Signature {
        channel_type: env::var("WALMART_CHANNEL_TYPE").unwrap(),
        consumer_id: env::var("WALMART_CONSUMER_ID").unwrap(),
        private_key: env::var("WALMART_PRIVATE_KEY").unwrap(),
      }
    },
  )
  .unwrap();

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
      ("list_wfs", m) => {
        order::list_wfs(&client, m.and_then(|m| m.value_of("STATUS")));
      }
      ("list_released", _) => {
        order::list_released(&client);
      }
      ("list_status", Some(m)) => {
        order::list_status(&client, m.value_of("STATUS").unwrap());
      }
      ("get", Some(m)) => {
        order::get(&client, m.value_of("ORDER_ID").unwrap());
      }
      ("dump", _) => {
        order::dump(&client);
      }
      ("ship", Some(m)) => {
        order::ship(&client, m);
      }
      ("ack", Some(m)) => {
        order::ack(&client, m.value_of("PO_ID").unwrap());
      }
      _ => {}
    },
    ("report", Some(matches)) => match matches.subcommand() {
      ("get", Some(matches)) => {
        report::get(&client, matches.value_of("report_type").unwrap());
      }
      ("get_raw", Some(matches)) => {
        report::get_raw(
          &client,
          matches.value_of("report_type").unwrap(),
          matches.value_of("out").unwrap(),
        );
      }
      _ => {}
    },
    ("item", Some(matches)) => match matches.subcommand() {
      ("dump", _) => {
        item::dump(&client);
      }
      ("dump_us", _) => {
        item::dump_us(&client);
      }
      _ => {}
    },
    ("inventory", Some(matches)) => match matches.subcommand() {
      ("set", Some(m)) => {
        let sku = m.value_of("sku").unwrap();
        let quantity = m.value_of("quantity").unwrap().parse().unwrap();
        let lagtime = m.value_of("lagtime").unwrap().parse().unwrap();
        inventory::set_inventory(&client, &sku, quantity, lagtime);
      }
      ("get", Some(m)) => {
        let sku = m.value_of("sku").unwrap();
        inventory::get_inventory(&client, &sku);
      }
      _ => {}
    },
    _ => {}
  }
}
