use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum CaReportCommand {
  Get(CaGet),
  GetRaw(CaGetRaw),
}

#[derive(Parser)]
pub struct CaGet {
  #[clap(long)]
  pub report_type: String,
}

#[derive(Parser)]
pub struct CaGetRaw {
  #[clap(long)]
  pub report_type: String,
  #[clap(long)]
  pub path: String,
}

#[derive(Subcommand)]
pub enum UsReportCommand {
  List(UsList),
  Get(UsGet),
  Create(UsCreate),
  GetDownload(UsGet),
}

#[derive(Parser)]
pub struct UsList {
  #[clap(long)]
  pub report_type: String,
  #[clap(long)]
  pub request_status: Option<String>,
  #[clap(long)]
  pub report_version: Option<String>,
  #[clap(long)]
  pub request_submission_start_date: Option<DateTime<Utc>>,
  #[clap(long)]
  pub request_submission_end_date: Option<DateTime<Utc>>,
}

#[derive(Parser)]
pub struct UsGet {
  #[clap(long)]
  pub report_request_id: String,
}

#[derive(Parser)]
pub struct UsCreate {
  #[clap(long)]
  pub report_type: String,
  #[clap(long)]
  pub report_version: String,
}

impl CaReportCommand {
  pub async fn run(self, client: walmart_partner_api::ca::Client) -> Result<()> {
    match self {
      CaReportCommand::Get(cmd) => {
        let r = match &*cmd.report_type {
          "item" => {
            client
              .get_report::<walmart_partner_api::ca::ItemReportType>()
              .await?
          }
          _ => unimplemented!(),
        };
        println!("{:#?}", r)
      }
      CaReportCommand::GetRaw(cmd) => {
        use std::fs::File;
        let out = File::create(cmd.path).unwrap();
        client
          .get_report_raw(
            walmart_partner_api::ca::GetReportQuery {
              type_: &cmd.report_type,
            },
            out,
          )
          .await?;
      }
    }
    Ok(())
  }
}

impl UsReportCommand {
  pub async fn run(self, client: walmart_partner_api::us::Client) -> Result<()> {
    match self {
      UsReportCommand::List(cmd) => {
        let r = client
          .get_all_report_requests(walmart_partner_api::us::GetAllReportRequestsQuery {
            report_type: cmd.report_type,
            request_status: cmd
              .request_status
              .map(|s| serde_json::from_str(&s).unwrap()),
            report_version: cmd.report_version,
            request_submission_start_date: cmd.request_submission_start_date,
            request_submission_end_date: cmd.request_submission_end_date,
          })
          .await?;
        println!("{:#?}", r)
      }
      UsReportCommand::Get(cmd) => {
        let r = client.get_report_request(cmd.report_request_id).await?;
        println!("{:#?}", r)
      }
      UsReportCommand::Create(cmd) => {
        let r = client
          .create_report_request(
            walmart_partner_api::us::CreateReportRequestQuery {
              report_type: cmd.report_type,
              report_version: cmd.report_version,
            },
            walmart_partner_api::us::CreateReportRequestInput {
              ..Default::default()
            },
          )
          .await?;
        println!("{:#?}", r)
      }
      UsReportCommand::GetDownload(cmd) => {
        let r = client.get_report_download(cmd.report_request_id).await?;
        println!("{:#?}", r)
      }
    }
    Ok(())
  }
}
