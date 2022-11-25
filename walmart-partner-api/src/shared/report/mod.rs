// pub use self::item::{ItemReport, ItemReportRow, ItemReportType};
// use crate::client::{Client, Method};
// use crate::result::*;
//
// use serde_urlencoded;
// use std::io::{Read, Write};
// mod item;
// use serde::{Deserialize, Serialize};
//
// pub trait ReportType {
//   type Data;
//
//   fn report_type() -> &'static str;
//   fn deserialize<R: Read>(r: R) -> WalmartResult<Self::Data>;
// }
//
// #[derive(Debug, Serialize, Default)]
// #[allow(non_snake_case)]
// pub struct GetReportQuery<'a> {
//   #[serde(rename = "type")]
//   pub type_: &'a str,
// }

// impl Client {
//   pub async fn get_report<R: ReportType>(&self) -> WalmartResult<R::Data> {
//     let qs = serde_urlencoded::to_string(&GetReportQuery {
//       type_: R::report_type(),
//     })?;
//     let res = self.send(self.req_json(Method::GET, "/v2/getReport", qs)?)?;
//     R::deserialize(res)
//   }
//   pub fn get_report_raw<W: Write>(&self, type_: &str, mut w: W) -> WalmartResult<u64> {
//     let qs = serde_urlencoded::to_string(&GetReportQuery { type_ })?;
//     let mut res = self
//       .send(self.req_json(Method::GET, "/v2/getReport", qs)?)?
//       .error_for_status()?;
//     res.copy_to(&mut w).map_err(Into::into)
//   }
// }
