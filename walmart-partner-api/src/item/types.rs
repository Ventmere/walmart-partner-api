use super::GetAllItemsQueryParams;
use crate::client::WalmartMarketplace;
use crate::result::*;
use crate::xml::*;
use serde::{Deserialize, Serialize};
use xmltree::Element;

/// Response of `get_all_items`
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct GetAllItems {
  pub items: Vec<Item>,

  /// US only
  pub totalItems: i64,
  /// US only
  pub nextCursor: Option<String>,
}

impl GetAllItems {
  /// As of 2018/10/30,
  /// `offset` works differently in USA
  /// `totalItems` is always 0 in Canada response
  /// `nextCursor` element never show up in Canada response
  pub(crate) fn get_next_query_params(
    &self,
    current_params: &GetAllItemsQueryParams,
    marketplace: WalmartMarketplace,
  ) -> Option<GetAllItemsQueryParams> {
    match marketplace {
      // use nextCursor
      WalmartMarketplace::USA => {
        self
          .nextCursor
          .as_ref()
          .map(|next_cursor| GetAllItemsQueryParams {
            nextCursor: next_cursor.to_string(),
            ..current_params.clone()
          })
      }
      // increase offset until the response has no items
      WalmartMarketplace::Canada => {
        if self.items.is_empty() {
          None
        } else {
          let limit = current_params.limit.clone().unwrap_or(20);
          Some(GetAllItemsQueryParams {
            offset: current_params.offset.or(Some(0)).map(|v| v + limit),
            ..current_params.clone()
          })
        }
      }
    }
  }
}

#[derive(Debug, Serialize, Default)]
pub struct Price {
  pub currency: String,
  pub amount: String,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Item {
  pub mart: String,
  pub sku: String,
  pub wpid: String,
  pub upc: String,
  pub gtin: String,
  pub productName: String,
  pub shelf: String,
  pub productType: String,
  pub price: Price,
  pub publishedStatus: String,
}

// impl FromXmlElement for GetAllItems {
//   fn from_xml_element(elem: Element) -> WalmartResult<Self> {
//     let items = elem
//       .children
//       .iter()
//       .filter_map(|c| {
//         if c.name == "ItemResponse" {
//           Some(Item {
//             mart: c.get_child_text_or_default("mart"),
//             sku: c.get_child_text_or_default("sku"),
//             wpid: c.get_child_text_or_default("wpid"),
//             upc: c.get_child_text_or_default("upc"),
//             gtin: c.get_child_text_or_default("gtin"),
//             productName: c.get_child_text_or_default("productName"),
//             shelf: c.get_child_text_or_default("shelf"),
//             productType: c.get_child_text_or_default("productType"),
//             price: c
//               .get_child("price")
//               .map(|c| Price {
//                 currency: c.get_child_text_or_default("currency"),
//                 amount: c.get_child_text_or_default("amount"),
//               })
//               .unwrap_or_default(),
//             publishedStatus: c.get_child_text_or_default("publishedStatus"),
//           })
//         } else {
//           None
//         }
//       })
//       .collect();
//     Ok(GetAllItems {
//       items,
//       totalItems: elem
//         .get_child_text_or_default("totalItems")
//         .parse()
//         .ok()
//         .unwrap_or_default(),
//       nextCursor: elem.get_child_text("nextCursor"),
//     })
//   }
// }
