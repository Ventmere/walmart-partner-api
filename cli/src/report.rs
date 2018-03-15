use walmart_partner_api::Client;
use walmart_partner_api::report::ItemReportType;

pub fn get(client: &Client, report_type: &str) {
  let report = match report_type {
    "item" => client.get_report::<ItemReportType>(),
    _ => unimplemented!()
  };
  println!("{:#?}", report);
}