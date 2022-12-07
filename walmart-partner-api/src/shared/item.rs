#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAllItems {
  /// Items included in the response list
  #[serde(rename = "ItemResponse")]
  #[serde(alias = "itemResponse")]
  #[serde(default)]
  pub item_response: Vec<Item>,
  /// Total items for the query
  #[serde(rename = "totalItems", skip_serializing_if = "Option::is_none")]
  pub total_items: Option<i64>,
  /// Used for pagination to fetch the next set of items
  #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
  pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetireItem {
  /// An arbitrary alphanumeric unique ID, specified by the seller, which identifies each item.
  #[serde(rename = "sku")]
  pub sku: String,
  /// Message confirming the deletion or retirement of an item from the Walmart Catalog
  #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
  /// The marketplace name. Example: Walmart_US
  #[serde(rename = "mart", skip_serializing_if = "Option::is_none")]
  pub mart: Option<String>,
  /// An arbitrary alphanumeric unique ID, specified by the seller, which identifies each item.
  #[serde(rename = "sku")]
  pub sku: String,
  /// The Walmart Product ID assigned by Walmart to the item when listed on Walmart.com
  #[serde(rename = "wpid", skip_serializing_if = "Option::is_none")]
  pub wpid: Option<String>,
  /// The 12-digit bar code used extensively for retail packaging in the United States
  #[serde(rename = "upc", skip_serializing_if = "Option::is_none")]
  pub upc: Option<String>,
  /// The GTIN-compatible Product ID (i.e. UPC or EAN). UPCs must be 12 or 14 digitis in length. EANs must be 13 digits in length.
  #[serde(rename = "gtin", skip_serializing_if = "Option::is_none")]
  pub gtin: Option<String>,
  /// A seller-specified, alphanumeric string uniquely identifying the product name. Example: 'Sterling Silver Blue Diamond Heart Pendant with 18in Chain'
  #[serde(rename = "productName", skip_serializing_if = "Option::is_none")]
  pub product_name: Option<String>,
  /// Walmart assigned an item shelf name
  #[serde(rename = "shelf", skip_serializing_if = "Option::is_none")]
  pub shelf: Option<String>,
  /// A seller-specified, alphanumeric string uniquely identifying the Product Type. Example: 'Diamond'
  #[serde(rename = "productType", skip_serializing_if = "Option::is_none")]
  pub product_type: Option<String>,
  #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
  pub price: Option<Price>,
  /// The status of an item when the item is in the submission process. The status can be one of the following: PUBLISHED, READY_TO_PUBLISH, IN_PROGRESS, UNPUBLISHED, STAGE, or SYSTEM_PROBLEM.
  #[serde(rename = "publishedStatus", skip_serializing_if = "Option::is_none")]
  pub published_status: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetItem {
  #[serde(rename = "ItemResponse")]
  #[serde(alias = "itemResponse")]
  pub item_response: Item,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
  pub currency: String,
  pub amount: String,
}
