extern crate chrono;
extern crate dotenv;
extern crate serde_json;
extern crate walmart_partner_api;

use std::env;

use anyhow::Result;
use clap::{command, Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use walmart_partner_api::WalmartCredential;

mod feed;
mod inventory;
mod item;
mod order;
mod report;

#[derive(Parser)]
#[command(author, version = "0.1", about = "Walmart CLI", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Commands,

  #[arg(short, long)]
  env: Option<String>,
}

#[derive(Parser)]
enum Commands {
  #[command(subcommand)]
  Ca(CaCommands),
  #[command(subcommand)]
  Us(UsCommands),
}

#[derive(Subcommand)]
enum CaCommands {
  #[command(subcommand)]
  Feed(feed::CaFeedCommand),
  #[command(subcommand)]
  Item(item::CaItemCommand),
  #[command(subcommand)]
  Report(report::CaReportCommand),
  #[command(subcommand)]
  Order(order::CaOrderCommand),
  #[command(subcommand)]
  Inventory(inventory::CaInventoryCommand),
}

#[derive(Subcommand)]
enum UsCommands {
  #[command(subcommand)]
  Feed(feed::UsFeedCommand),
  #[command(subcommand)]
  Report(report::UsReportCommand),
  #[command(subcommand)]
  Order(order::UsOrderCommand),
  #[command(subcommand)]
  Inventory(inventory::UsInventoryCommand),
  #[command(subcommand)]
  Item(item::UsItemCommand),
}

impl CaCommands {
  pub async fn run(self, client: walmart_partner_api::ca::Client) -> Result<()> {
    match self {
      CaCommands::Feed(cmd) => cmd.run(client).await,
      CaCommands::Item(cmd) => cmd.run(client).await,
      CaCommands::Report(cmd) => cmd.run(client).await,
      CaCommands::Order(cmd) => cmd.run(client).await,
      CaCommands::Inventory(cmd) => cmd.run(client).await,
    }
  }
}

impl UsCommands {
  pub async fn run(self, client: walmart_partner_api::us::Client) -> Result<()> {
    match self {
      UsCommands::Feed(cmd) => cmd.run(client).await,
      UsCommands::Report(cmd) => cmd.run(client).await,
      UsCommands::Order(cmd) => cmd.run(client).await,
      UsCommands::Inventory(cmd) => cmd.run(client).await,
      UsCommands::Item(cmd) => cmd.run(client).await,
    }
  }
}

#[tokio::main]
async fn main() {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::TRACE)
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

  let cli = Cli::parse();
  if let Some(path) = cli.env.as_deref() {
    dotenv::from_path(::std::path::Path::new(path)).unwrap();
  } else {
    dotenv::dotenv().unwrap();
  }

  let credentials = if env::var("WALMART_CLIENT_ID").ok().is_some() {
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
  };
  match cli.command {
    Commands::Ca(cmd) => {
      let client = walmart_partner_api::ca::Client::new(credentials).unwrap();
      cmd.run(client).await.unwrap();
    }
    Commands::Us(cmd) => {
      let client = walmart_partner_api::us::Client::new(credentials).unwrap();
      cmd.run(client).await.unwrap();
    }
  }
}
