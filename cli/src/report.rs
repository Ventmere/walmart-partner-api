use walmart_partner_api::report::ItemReportType;
use walmart_partner_api::Client;

pub fn get(client: &Client, report_type: &str) {
  let report = match report_type {
    "item" => client.get_report::<ItemReportType>(),
    _ => unimplemented!(),
  };
  println!("{:#?}", report);
}

pub fn get_raw(client: &Client, report_type: &str, path: &str) {
  use std::fs::File;
  let out = File::create(path).unwrap();
  client.get_report_raw(report_type, out).unwrap();
}
