use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum CaItemCommand {
  Get(Get),
  List(List),
  Retire(Retire),
}

#[derive(Subcommand)]
pub enum UsItemCommand {
  Get(Get),
  List(List),
  Retire(Retire),
}

#[derive(Parser)]
pub struct Get {
  #[clap(long)]
  pub sku: String,
}

#[derive(Parser)]
pub struct List {
  #[clap(long)]
  pub next_cursor: Option<String>,
  #[clap(long)]
  pub sku: Option<String>,
  #[clap(long)]
  pub limit: Option<i32>,
  #[clap(long)]
  pub offset: Option<i32>,
}

#[derive(Parser)]
pub struct Retire {
  #[clap(long)]
  pub sku: String,
}

impl CaItemCommand {
  pub async fn run(self, client: walmart_partner_api::ca::Client) -> Result<()> {
    match self {
      CaItemCommand::Get(cmd) => {
        let r = client.get_item(&cmd.sku).await?;
        println!("{:#?}", r)
      }
      CaItemCommand::List(cmd) => {
        let r = client
          .get_all_items(walmart_partner_api::ca::GetAllItemsQuery {
            next_cursor: cmd.next_cursor,
            sku: cmd.sku,
            limit: cmd.limit,
            offset: cmd.offset,
          })
          .await?;
        println!("{:#?}", r)
      }
      CaItemCommand::Retire(cmd) => {
        let r = client.retire_item(&cmd.sku).await?;
        println!("{:#?}", r)
      }
    }
    Ok(())
  }
}

impl UsItemCommand {
  pub async fn run(self, client: walmart_partner_api::us::Client) -> Result<()> {
    match self {
      UsItemCommand::Get(cmd) => {
        let r = client.get_item(&cmd.sku).await?;
        println!("{:#?}", r)
      }
      UsItemCommand::List(cmd) => {
        let r = client
          .get_all_items(walmart_partner_api::us::GetAllItemsQuery {
            next_cursor: cmd.next_cursor,
            sku: cmd.sku,
            limit: cmd.limit,
            offset: cmd.offset,
            ..Default::default()
          })
          .await?;
        println!("{:#?}", r)
      }
      UsItemCommand::Retire(cmd) => {
        let r = client.retire_item(&cmd.sku).await?;
        println!("{:#?}", r)
      }
    }
    Ok(())
  }
}
