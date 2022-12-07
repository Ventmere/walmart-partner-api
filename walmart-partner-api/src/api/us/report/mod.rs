use chrono::{DateTime, Utc};
use serde::Serialize;

pub use types::*;

use crate::api::us::Client;
use crate::client::Method;
use crate::result::WalmartResult;

mod types;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllReportRequestsQuery {
  pub report_type: String,
  pub report_version: Option<String>,
  pub request_status: Option<ReportRequestStatus>,
  pub request_submission_start_date: Option<DateTime<Utc>>,
  pub request_submission_end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReportRequestQuery {
  pub report_type: String,
  pub report_version: String,
}

impl Client {
  pub async fn get_all_report_requests(
    &self,
    query: GetAllReportRequestsQuery,
  ) -> WalmartResult<ReportRequestList> {
    let qs = serde_urlencoded::to_string(&query)?;
    let req = self.req_json(Method::GET, "/v3/reports/reportRequests", qs)?;
    self.send(req).await?.res_json().await
  }

  pub async fn get_report_request(
    &self,
    report_request_id: impl AsRef<str>,
  ) -> WalmartResult<ReportRequest> {
    let req = self.req_json(
      Method::GET,
      &format!("/v3/reports/reportRequests/{}", report_request_id.as_ref()),
      (),
    )?;
    self.send(req).await?.res_json().await
  }

  pub async fn create_report_request(
    &self,
    query: CreateReportRequestQuery,
    input: CreateReportRequestInput,
  ) -> WalmartResult<ReportRequest> {
    let qs = serde_urlencoded::to_string(&query)?;
    let req = self
      .req_json(Method::POST, "/v3/reports/reportRequests", qs)?
      .body_json(&input)?;
    self.send(req).await?.res_json().await
  }

  pub async fn get_report_download(
    &self,
    report_request_id: impl AsRef<str>,
  ) -> WalmartResult<ReportDownload> {
    let req = self.req_json(
      Method::GET,
      "/v3/reports/downloadReport",
      vec![("requestId", report_request_id.as_ref())],
    )?;
    self.send(req).await?.res_json().await
  }
}
