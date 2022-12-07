use std::fs;
use std::io;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

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
  Generate { country: Country, api_name: ApiName },
}

#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[serde(rename_all = "lowercase")]
enum Country {
  Us,
  Ca,
}

impl ToString for Country {
  fn to_string(&self) -> String {
    serde_json::to_value(self)
      .unwrap()
      .as_str()
      .unwrap()
      .to_string()
  }
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
  OnRequestReports,
}

impl ToString for ApiName {
  fn to_string(&self) -> String {
    serde_json::to_value(self)
      .unwrap()
      .as_str()
      .unwrap()
      .to_string()
  }
}

fn download_schema_file(country: Country, api_name: ApiName) -> Result<PathBuf> {
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

  let resolver = PathResolver::new(country, api_name);
  resolver.create_schema_dir()?;
  let (path, mut file) = resolver.create_schema_file()?;

  io::copy(&mut response, &mut file).context("Failed to copy reqwest body to schema file")?;
  Ok(path)
}

#[derive(Clone)]
struct PathResolver {
  country: Country,
  api_name: ApiName,
}

impl PathResolver {
  fn new(country: Country, api_name: ApiName) -> Self {
    Self { country, api_name }
  }

  fn get_current_dir() -> PathBuf {
    let path = std::env::current_dir().unwrap();
    if !path.ends_with("walmart-partner-api") {
      panic!(
        "Make sure you run this command in the root directory of the project (walmart-partner-api)"
      );
    }
    path
  }

  fn create_schema_file(&self) -> Result<(PathBuf, fs::File)> {
    let path = self.get_schema_file_path();
    let file = fs::File::create(path.clone())
      .context(format!("Failed to create schema file for {:?}", path))?;
    Ok((path, file))
  }

  fn create_schema_dir(&self) -> Result<PathBuf> {
    let path = self.get_country_schema_dir();
    fs::create_dir_all(&path).context(format!("Failed to create schema directory: {:?}", path))?;
    Ok(path)
  }

  fn get_schema_file_path(&self) -> PathBuf {
    let path = self
      .get_country_schema_dir()
      .join(format!("{}.json", self.api_name.to_string()));
    path
  }

  fn get_country_schema_dir(&self) -> PathBuf {
    let path = Self::get_current_dir()
      .join("schemas")
      .join(self.country.to_string());
    path
  }
}

fn generate(_country: Country, _api_name: ApiName) -> Result<()> {
  unimplemented!();

  // Command::new("openapi-generator")
  //   .arg("generate")
  //   .arg("-i")
  //   .arg(path.to_str().unwrap_or_default())
  //   .arg("--skip-validate-spec")
  //   .arg("--global-property")
  //   .arg("models,modelDocs=false")
  //   .arg("-g")
  //   .arg("rust")
  //   .arg("-o")
  //   .arg("tmp")
  //   .status()
  //   .context("Failed to execute openapi-generator command")?;
}

fn main() {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Download { country, api_name } => {
      download_schema_file(*country, *api_name).unwrap();
    }
    Commands::Generate { country, api_name } => {
      generate(*country, *api_name).unwrap();
    }
  }
}
