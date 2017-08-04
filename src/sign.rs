use reqwest::Method;
use ring::{rand, signature};
use chrono::{DateTime, Utc};

use error::Result;

pub struct Signature {
  consumer_id: String,
  private_key: signature::RSAKeyPair,

}

impl Signature {
  pub new() -> Result<Signature> {

  }

  pub fn sign(url: &str, method: Method) {

  }
}

pub struct Signed {
  pub result: String,
  pub timestamp: DateTime<Utc>,
}