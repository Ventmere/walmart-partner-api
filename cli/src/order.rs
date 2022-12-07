use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum CaOrderCommand {
  List(List),
  ListReleased(ListReleased),
  Get(Get),
  Ack(Get),
  Ship(CaShip),
}

#[derive(Subcommand)]
pub enum UsOrderCommand {
  List(List),
  ListReleased(ListReleased),
  Get(Get),
  Ack(Get),
  Ship(UsShip),
}

#[derive(Parser)]
pub struct Get {
  #[clap(long)]
  pub purchase_order_id: String,
}

#[derive(Parser)]
pub struct List {
  #[clap(long)]
  pub sku: Option<String>,
  #[clap(long)]
  pub customer_order_id: Option<String>,
  #[clap(long)]
  pub purchase_order_id: Option<String>,
  #[clap(long)]
  pub status: Option<String>,
  #[clap(long)]
  pub created_start_date: Option<DateTime<Utc>>,
  #[clap(long)]
  pub created_end_date: Option<DateTime<Utc>>,
  #[clap(long)]
  pub from_expected_ship_date: Option<DateTime<Utc>>,
  #[clap(long)]
  pub to_expected_ship_date: Option<DateTime<Utc>>,
  #[clap(long)]
  pub limit: Option<i32>,
  #[clap(long)]
  pub product_info: Option<bool>,
}

#[derive(Parser)]
pub struct ListReleased {
  #[clap(long)]
  pub limit: Option<i32>,
  #[clap(long)]
  pub created_start_date: Option<DateTime<Utc>>,
  #[clap(long)]
  pub created_end_date: Option<DateTime<Utc>>,
  #[clap(long)]
  pub product_info: Option<bool>,
}

#[derive(Parser)]
pub struct CaShip {
  #[clap(long)]
  pub purchase_order_id: String,
  #[clap(long)]
  pub line_number: String,
  #[clap(long)]
  pub ship_from_country: String,
  #[clap(long)]
  pub ship_date_time: DateTime<Utc>,
  #[clap(long)]
  pub other_carrier: Option<String>,
  #[clap(long)]
  pub carrier: Option<String>,
  #[clap(long)]
  pub method_code: String,
  #[clap(long)]
  pub tracking_number: String,
  #[clap(long)]
  pub tracking_url: Option<String>,
}

#[derive(Parser)]
pub struct UsShip {
  #[clap(long)]
  pub purchase_order_id: String,
  #[clap(long)]
  pub line_number: String,
  #[clap(long)]
  pub seller_order_id: String,
  #[clap(long)]
  pub ship_date_time: DateTime<Utc>,
  #[clap(long)]
  pub other_carrier: Option<String>,
  #[clap(long)]
  pub carrier: Option<String>,
  #[clap(long)]
  pub method_code: String,
  #[clap(long)]
  pub tracking_number: String,
  #[clap(long)]
  pub tracking_url: Option<String>,
}

impl CaOrderCommand {
  pub async fn run(self, client: walmart_partner_api::ca::Client) -> Result<()> {
    match self {
      CaOrderCommand::List(cmd) => {
        let r = client
          .get_all_orders(walmart_partner_api::ca::GetAllOrdersQuery {
            sku: cmd.sku,
            customer_order_id: cmd.customer_order_id,
            purchase_order_id: cmd.purchase_order_id,
            status: cmd.status,
            created_start_date: cmd.created_start_date.unwrap_or_default(),
            created_end_date: cmd.created_end_date,
            from_expected_ship_date: cmd.from_expected_ship_date,
            to_expected_ship_date: cmd.to_expected_ship_date,
            limit: cmd.limit,
            product_info: cmd.product_info,
          })
          .await?;
        println!("{:#?}", r)
      }
      CaOrderCommand::ListReleased(cmd) => {
        let r = client
          .get_all_released_orders(walmart_partner_api::ca::GetAllReleasedOrdersQuery {
            limit: cmd.limit,
            created_start_date: cmd.created_start_date.unwrap_or_default(),
            created_end_date: cmd.created_end_date,
            product_info: cmd.product_info,
          })
          .await?;
        println!("{:#?}", r)
      }
      CaOrderCommand::Get(cmd) => {
        let r = client
          .get_order(&cmd.purchase_order_id, Default::default())
          .await?;
        println!("{:#?}", r)
      }
      CaOrderCommand::Ack(cmd) => {
        let r = client.ack_order(&cmd.purchase_order_id).await?;
        println!("{:#?}", r)
      }
      CaOrderCommand::Ship(cmd) => {
        let r = client
          .ship_order_lines(
            cmd.purchase_order_id,
            walmart_partner_api::ca::ShipOrderLines {
              order_lines: vec![walmart_partner_api::ca::ShipOrderLine {
                line_number: cmd.line_number,
                ship_from_country: cmd.ship_from_country,
                status_quantity: None,
                asn: None,
                tracking_info: walmart_partner_api::ca::OrderLineTrackingInfo {
                  ship_date_time: Default::default(),
                  carrier_name: walmart_partner_api::ca::OrderLineTrackingCarrier {
                    other_carrier: cmd.other_carrier,
                    carrier: cmd.carrier,
                  },
                  method_code: cmd.method_code,
                  tracking_number: cmd.tracking_number,
                  tracking_url: cmd.tracking_url,
                },
              }],
            },
          )
          .await?;
        println!("{:#?}", r)
      }
    }
    Ok(())
  }
}

impl UsOrderCommand {
  pub async fn run(self, client: walmart_partner_api::us::Client) -> Result<()> {
    match self {
      UsOrderCommand::List(cmd) => {
        let r = client
          .get_all_orders(walmart_partner_api::us::GetAllOrdersQuery {
            sku: cmd.sku,
            customer_order_id: cmd.customer_order_id,
            purchase_order_id: cmd.purchase_order_id,
            status: cmd.status,
            created_start_date: cmd.created_start_date,
            created_end_date: cmd.created_end_date,
            from_expected_ship_date: cmd.from_expected_ship_date,
            to_expected_ship_date: cmd.to_expected_ship_date,
            limit: cmd.limit,
            product_info: cmd.product_info,
            ship_node_type: None,
            replacement_into: None,
            order_type: None,
          })
          .await?;
        println!("{:#?}", r)
      }
      UsOrderCommand::ListReleased(cmd) => {
        let r = client
          .get_all_released_orders(walmart_partner_api::us::GetAllReleasedOrdersQuery {
            limit: cmd.limit,
            created_start_date: cmd.created_start_date,
            created_end_date: cmd.created_end_date,
            product_info: cmd.product_info,
            sku: None,
            customer_order_id: None,
            purchase_order_id: None,
            from_expected_ship_date: None,
            to_expected_ship_date: None,
            replacement_into: None,
            order_type: None,
            ship_node_type: None,
          })
          .await?;
        println!("{:#?}", r)
      }
      UsOrderCommand::Get(cmd) => {
        let r = client
          .get_order(&cmd.purchase_order_id, Default::default())
          .await?;
        println!("{:#?}", r)
      }
      UsOrderCommand::Ack(cmd) => {
        let r = client.ack_order(&cmd.purchase_order_id).await?;
        println!("{:#?}", r)
      }
      UsOrderCommand::Ship(cmd) => {
        let r = client
          .ship_order_lines(
            cmd.purchase_order_id,
            walmart_partner_api::us::ShipOrderLines {
              order_lines: vec![walmart_partner_api::us::ShipOrderLine {
                line_number: cmd.line_number,
                seller_order_id: cmd.seller_order_id,
                intent_to_cancel_override: None,
                status_quantity: None,
                asn: None,
                tracking_info: walmart_partner_api::us::OrderLineTrackingInfo {
                  ship_date_time: cmd.ship_date_time,
                  carrier_name: walmart_partner_api::us::OrderLineTrackingCarrier {
                    other_carrier: cmd.other_carrier,
                    carrier: cmd.carrier,
                  },
                  method_code: cmd.method_code,
                  tracking_number: cmd.tracking_number,
                  tracking_url: cmd.tracking_url,
                },
              }],
              process_mode: None,
            },
          )
          .await?;
        println!("{:#?}", r)
      }
    }
    Ok(())
  }
}
