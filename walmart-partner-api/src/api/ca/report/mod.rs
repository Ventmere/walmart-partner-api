use std::io::Write;

use serde_urlencoded;

pub use types::*;

use crate::client::{Client, Method};
use crate::result::*;

mod types;

#[derive(Debug, Default, Serialize)]
#[allow(non_snake_case)]
pub struct GetReportQuery<'a> {
  #[serde(rename = "type")]
  pub type_: &'a str,
}

impl Client {
  pub async fn get_report<R: ReportType>(&self) -> WalmartResult<R::Data> {
    let qs = serde_urlencoded::to_string(&GetReportQuery {
      type_: R::report_type(),
    })?;
    let req = self.req_xml(Method::GET, "/v3/getReport", qs)?;
    let res = self.send(req).await?.res_bytes().await?;
    R::deserialize(&*res)
  }

  pub async fn get_report_raw<W: Write>(
    &self,
    query: GetReportQuery<'_>,
    mut w: W,
  ) -> WalmartResult<u64> {
    let qs = serde_urlencoded::to_string(&query)?;
    let req = self.req_xml(Method::GET, "/v3/getReport", qs)?;
    let res = self.send(req).await?.res_bytes().await?;
    std::io::copy(&mut &*res, &mut w).map_err(Into::into)
  }
}
