use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum CaInventoryCommand {
  Get(Get),
  Update(CaUpdate),
}

#[derive(Subcommand)]
pub enum UsInventoryCommand {
  Get(Get),
  Update(UsUpdate),
}

#[derive(Parser)]
pub struct Get {
  #[clap(long)]
  pub sku: String,
}

#[derive(Parser)]
pub struct CaUpdate {
  #[clap(long)]
  pub sku: String,
  #[clap(long)]
  pub unit: String,
  #[clap(long)]
  pub amount: i32,
  #[clap(long)]
  pub fulfillment_lag_time: i32,
  #[clap(long)]
  pub partner_id: Option<String>,
  #[clap(long)]
  pub offer_id: Option<String>,
}

#[derive(Parser)]
pub struct UsUpdate {
  #[clap(long)]
  pub sku: String,
  #[clap(long)]
  pub unit: String,
  #[clap(long)]
  pub amount: i32,
}

impl CaInventoryCommand {
  pub async fn run(self, client: walmart_partner_api::ca::Client) -> Result<()> {
    match self {
      CaInventoryCommand::Get(cmd) => {
        let r = client.get_item_inventory(cmd.sku).await?;
        println!("{:#?}", r)
      }
      CaInventoryCommand::Update(cmd) => {
        let r = client
          .update_item_inventory(walmart_partner_api::ca::Inventory {
            sku: cmd.sku,
            fulfillment_lag_time: cmd.fulfillment_lag_time,
            partner_id: cmd.partner_id,
            offer_id: cmd.offer_id,
            quantity: walmart_partner_api::ca::InventoryQuantity {
              unit: cmd.unit,
              amount: cmd.amount,
            },
          })
          .await?;
        println!("{:#?}", r)
      }
    }
    Ok(())
  }
}

impl UsInventoryCommand {
  pub async fn run(self, client: walmart_partner_api::us::Client) -> Result<()> {
    match self {
      UsInventoryCommand::Get(cmd) => {
        let r = client.get_item_inventory(cmd.sku).await?;
        println!("{:#?}", r)
      }
      UsInventoryCommand::Update(cmd) => {
        let r = client
          .update_item_inventory(walmart_partner_api::us::Inventory {
            sku: cmd.sku,
            quantity: walmart_partner_api::us::InventoryQuantity {
              unit: cmd.unit,
              amount: cmd.amount,
            },
          })
          .await?;
        println!("{:#?}", r)
      }
    }
    Ok(())
  }
}
