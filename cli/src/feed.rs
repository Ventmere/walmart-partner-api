use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum CaFeedCommand {
  ListStatuses(FeedStatus),
  GetStatusItem(FeedStatusItem),
  UploadXml(CaUploadXml),
}

#[derive(Subcommand)]
pub enum UsFeedCommand {
  ListStatuses(FeedStatus),
  GetStatusItem(FeedStatusItem),
}

#[derive(Parser)]
pub struct FeedStatus {
  #[clap(long)]
  pub feed_id: Option<String>,
  #[clap(long)]
  pub limit: Option<i32>,
  #[clap(long)]
  pub offset: Option<i32>,
}

#[derive(Parser)]
pub struct FeedStatusItem {
  #[clap(long)]
  pub id: String,
  #[clap(long)]
  pub include_details: Option<bool>,
  #[clap(long)]
  pub limit: Option<i32>,
  #[clap(long)]
  pub offset: Option<i32>,
}

#[derive(Parser)]
pub struct CaUploadXml {
  #[clap(long)]
  pub feed_type: String,
  #[clap(long)]
  pub path: String,
}

impl CaFeedCommand {
  pub async fn run(self, client: walmart_partner_api::ca::Client) -> Result<()> {
    match self {
      CaFeedCommand::ListStatuses(cmd) => {
        let r = client
          .get_all_feed_statuses(walmart_partner_api::ca::GetAllFeedStatusesQuery {
            feed_id: cmd.feed_id,
            limit: cmd.limit,
            offset: cmd.offset,
          })
          .await?;
        println!("{:#?}", r)
      }
      CaFeedCommand::GetStatusItem(cmd) => {
        let r = client
          .get_feed_and_item_status(
            &cmd.id,
            walmart_partner_api::ca::GetFeedAndItemStatusQuery {
              include_details: cmd.include_details,
              limit: cmd.limit,
              offset: cmd.offset,
            },
          )
          .await?;
        println!("{:#?}", r)
      }
      CaFeedCommand::UploadXml(cmd) => {
        let f = std::fs::File::open(cmd.path).unwrap();
        let r = client.bulk_upload_xml(&cmd.feed_type, f).await?;
        println!("{:#?}", r)
      }
    }
    Ok(())
  }
}

impl UsFeedCommand {
  pub async fn run(self, client: walmart_partner_api::us::Client) -> Result<()> {
    match self {
      UsFeedCommand::ListStatuses(cmd) => {
        let r = client
          .get_all_feed_statuses(walmart_partner_api::us::GetAllFeedStatusesQuery {
            feed_id: cmd.feed_id,
            limit: cmd.limit,
            offset: cmd.offset,
          })
          .await?;
        println!("{:#?}", r)
      }
      UsFeedCommand::GetStatusItem(cmd) => {
        let r = client
          .get_feed_and_item_status(
            &cmd.id,
            walmart_partner_api::us::GetFeedAndItemStatusQuery {
              include_details: cmd.include_details,
              limit: cmd.limit,
              offset: cmd.offset,
            },
          )
          .await?;
        println!("{:#?}", r)
      }
    }
    Ok(())
  }
}
