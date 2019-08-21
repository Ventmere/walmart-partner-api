use super::ReportType;
use crate::result::*;
use bigdecimal::BigDecimal;
use csv;
use std::io::prelude::*;
use std::io::BufReader;
use tempfile;
use zip::read::ZipArchive;

pub struct ItemReportType;

#[derive(Debug)]
pub struct ItemReport {
  pub rows: Vec<ItemReportRow>,
}

macro_rules! impl_from_csv {
  (
    pub struct $name:ident {
      $(
        #[csv(header = $header:expr $(, $modifier:ident)*)]
        pub $field:ident: $ty:ty,
      )*
    }
  ) => {
    #[derive(Debug)]
    pub struct $name {
      $(pub $field: $ty,)*
    }

    impl $name {
      fn from_csv(headers: &[String], row: &[&str]) -> WalmartResult<Self> {
        $(
          let mut $field = None;
        )*

        for (i, v) in row.into_iter().enumerate() {
          match headers.get(i).map(AsRef::as_ref) {
            $(
              Some($header) => {
                $field = v.parse().map_err(|err| -> WalmartError {
                  format!("Parse field '{}' error: {}", $header, err).into()
                }).ok()
              }
            )*,
            Some(unknown_header) => {
              return Err(
                format!("Unknown header '{}'.", unknown_header).into()
              )
            }
            None => return Err(
              format!("Column index {} is out of bound.", i).into()
            ),
          }
        }

        Ok(Self {
          $(
            $field: impl_from_csv!(GET_VALUE $field, $header, $($modifier),*),
          )*
        })
      }
    }
  };

  (GET_VALUE $field:ident, $header:expr, ) => {
    $field.ok_or_else(|| -> WalmartError {
      format!("Field '{}' was not found.", $header).into()
    })?
  };

  (GET_VALUE $field:ident, $header:expr, optional) => {
    $field
  };
}

impl_from_csv! {
  pub struct ItemReportRow {
    #[csv(header = "PARTNER ID")]
    pub partner_id: String,
    #[csv(header = "SKU")]
    pub sku: String,
    #[csv(header = "PRODUCT NAME")]
    pub product_name: String,
    #[csv(header = "PRODUCT CATEGORY")]
    pub product_category: String,
    #[csv(header = "PRICE")]
    pub price: BigDecimal,
    #[csv(header = "CURRENCY")]
    pub currency: String,
    #[csv(header = "PUBLISH STATUS")]
    pub publish_status: String,
    #[csv(header = "STATUS CHANGE REASON")]
    pub status_change_reason: String,
    #[csv(header = "LIFECYCLE STATUS")]
    pub lifecycle_status: String,
    #[csv(header = "INVENTORY COUNT", optional)]
    pub inventory_count: Option<i64>,
    #[csv(header = "SHIP METHODS")]
    pub ship_methods: String,
    #[csv(header = "WPID")]
    pub wpid: String,
    #[csv(header = "ITEM ID")]
    pub item_id: String,
    #[csv(header = "GTIN")]
    pub gtin: String,
    #[csv(header = "UPC")]
    pub upc: String,
    #[csv(header = "PRIMARY IMAGE URL")]
    pub primary_image_url: String,
    #[csv(header = "SHELF NAME")]
    pub shelf_name: String,
    #[csv(header = "PRIMARY CAT PATH")]
    pub primary_cat_path: String,
    #[csv(header = "OFFER START DATE")]
    pub offer_start_date: String,
    #[csv(header = "OFFER END DATE")]
    pub offer_end_date: String,
    #[csv(header = "ITEM CREATION DATE")]
    pub item_creation_date: String,
    #[csv(header = "ITEM LAST UPDATED")]
    pub item_last_updated: String,
    #[csv(header = "ITEM PAGE URL")]
    pub item_page_url: String,
    #[csv(header = "REVIEWS COUNT")]
    pub reviews_count: i64,
    #[csv(header = "AVERAGE RATING", optional)]
    pub average_rating: Option<i64>,
    #[csv(header = "SEARCHABLE?")]
    pub searchable: String,
  }
}

impl ReportType for ItemReportType {
  type Data = ItemReport;
  fn report_type() -> &'static str {
    "item"
  }

  fn deserialize<R: Read>(mut r: R) -> WalmartResult<Self::Data> {
    use std::io::{copy, SeekFrom};

    let mut tmp_file = tempfile::tempfile()?;
    copy(&mut r, &mut tmp_file)?;
    tmp_file.seek(SeekFrom::Start(0))?;
    let mut br = BufReader::new(&mut tmp_file);
    let mut zip = ZipArchive::new(&mut br)?;
    let csv_file = zip.by_index(0)?;
    let mut csv_reader = csv::Reader::from_reader(csv_file);
    let headers: Vec<String> = csv_reader
      .headers()
      .map_err(|err| -> WalmartError { format!("Parse csv header error: {}", err).into() })?
      .iter()
      .map(|s| s.to_string())
      .collect();
    let mut rows = vec![];
    for (i, result) in csv_reader.records().enumerate() {
      let record = result.map_err(|err| -> WalmartError {
        format!("Line {}: Parse csv error: {}", i + 1, err).into()
      })?;
      let fields: Vec<&str> = record.into_iter().collect();
      rows.push(ItemReportRow::from_csv(&headers, &fields)?)
    }

    Ok(ItemReport { rows })
  }
}

// #[test]
// fn test_item_report_deserialize() {
//   use std::io::prelude::*;
//   use std::fs::File;

//   let mut f = File::open("../target/report.zip").unwrap();
//   let report = ItemReportType::deserialize(f).unwrap();
//   println!("{:#?}", report.rows)
// }
