use error::*;
use serde_urlencoded;
use std::io::{Read, Write};
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
    let res = self
      .request_json(Method::Get, "/v2/getReport", qs)?
      .send()?;
    R::deserialize(res)
  }
  pub fn get_report_raw<W: Write>(&self, type_: &str, mut w: W) -> Result<u64> {
    let qs = serde_urlencoded::to_string(&GetReportQuery { type_ })?;
    let mut res = self
      .request_json(Method::Get, "/v2/getReport", qs)?
      .send()?
      .error_for_status()?;
    res.copy_to(&mut w).map_err(Into::into)
  }
}
