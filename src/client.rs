use reqwest;
pub use reqwest::{Method, Response};
use error::*;
use sign::Signature;

const BASE_URL: &'static str = "https://marketplace.walmartapis.com";
const CHANNEL_TYPE: &'static str = "0f3e4dd4-0514-4346-b39d-af0e00ea066d";

pub struct Client {
  sign: Signature,
  http: reqwest::Client,
}

impl Client {
  pub fn new(consumer_id: &str, private_key: &str) -> Result<Client> {
    let http = reqwest::Client::new()?;
    Client::with_http_client(consumer_id, private_key, http)
  }

  pub fn with_http_client(consumer_id: &str, private_key: &str, http: reqwest::Client) -> Result<Client> {
    Ok(Client {
      sign: Signature::new(consumer_id, private_key)?,
      http: http,
    })
  }

  pub fn request_json() -> Result<Response> {
    unimplemented!()
  }

  pub fn request_xml() -> Result<Response> {
    unimplemented!()
  }
}