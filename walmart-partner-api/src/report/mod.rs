use error::*;
use std::io::Read;
use serde_urlencoded;
mod item;

pub use self::item::{ItemReport, ItemReportRow, ItemReportType};
use client::{Client, Method};

pub trait ReportType {
  type Data;

  fn report_type() -> &'static str;
  fn deserialize<R: Read>(r: R) -> Result<Self::Data>;
}

#[derive(Debug, Serialize, Default)]
#[allow(non_snake_case)]
pub struct GetReportQuery<'a> {
  #[serde(rename = "type")]
  pub type_: &'a str,
}

impl Client {
  pub fn get_report<R: ReportType>(&self) -> Result<R::Data> {
    let qs = serde_urlencoded::to_string(&GetReportQuery {
      type_: R::report_type(),
    })?;
    let res = self.request_json(Method::Get, "/v2/getReport", qs)?.send()?;
    R::deserialize(res)
  }
}
