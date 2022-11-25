use std::fmt::format;
use std::fs;
use std::io;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Download { country: Country, api_name: ApiName },
}

#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[serde(rename_all = "lowercase")]
enum Country {
  Us,
  Ca,
}

#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[serde(rename_all = "lowercase")]
enum ApiName {
  Auth,
  Feeds,
  Items,
  Price,
  Promotion,
  Orders,
  Returns,
  Inventory,
  Settings,
  Rules,
  Reports,
  Fulfillment,
  Notifications,
  Utilities,
  Insights,
}

fn download_schema_files(country: Country, api_name: ApiName) {
  #[derive(Debug, Serialize)]
  struct Body {
    params: Params,
  }
  #[derive(Debug, Serialize)]
  #[serde(rename_all = "camelCase")]
  struct Params {
    country: Country,
    api_name: ApiName,
    category: &'static str,
  }

  let body = Body {
    params: Params {
      country,
      api_name,
      category: "mp",
    },
  };

  let client = reqwest::blocking::Client::new();
  let mut response = client
    .post("https://developer.walmart.com/api/detail")
    .json(&body)
    .send()
    .unwrap()
    .error_for_status()
    .unwrap();

  let country = serde_json::to_value(&country)
    .unwrap()
    .as_str()
    .unwrap()
    .to_string();
  let api_name = serde_json::to_value(&api_name)
    .unwrap()
    .as_str()
    .unwrap()
    .to_string();

  let dir_path = PathBuf::from(format!("../schemas/{}", country));
  fs::create_dir_all(&dir_path).unwrap();

  let file_path = dir_path.join(format!("{}.json", api_name));
  let mut file = fs::File::create(file_path).unwrap();
  io::copy(&mut response, &mut file).unwrap();
}

fn main() {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Download { country, api_name } => {
      download_schema_files(*country, *api_name);
    }
  }
}
