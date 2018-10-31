use serde_json;
use walmart_partner_api::item::*;
use walmart_partner_api::Client;

pub fn dump(client: &Client) {
  let mut params: GetAllItemsQueryParams = Default::default();
  params.limit = Some(20);
  let mut items = vec![];
  loop {
    println!("loading params = {:#?}", params);

    let (res, next_params) = client.get_all_items(&params).unwrap();
    let mut res = res.into_inner();

    if res.items.is_empty() {
      break;
    }

    if let Some(next_params) = next_params {
      params = next_params;
      println!("page items = {}", res.items.len());
      items.append(&mut res.items);
    } else {
      break;
    }
  }

  println!("totalItems = {}", items.len());

  println!("{}", serde_json::to_string_pretty(&items).unwrap());
}
