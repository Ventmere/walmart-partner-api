//! Implement Walmart's authentication signature
//!
//! [Walmart Documentation](https://developer.walmart.com/#/apicenter/contentProvider#authentication)

pub use reqwest::Method;
use openssl::rsa::Rsa;
use openssl::sign::Signer;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use base64::{encode};

use error::*;

pub struct Signature {
  consumer_id: String,
  keypair: PKey,
}

impl Signature {
  /// Cosntruct a new `Signature`
  pub fn new(consumer_id: &str, private_key: &str) -> Result<Signature> {
    let pem_key = format!("-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----", private_key);
    let keypair = Rsa::private_key_from_pem(pem_key.as_bytes())?;
    let keypair = PKey::from_rsa(keypair)?;
    Ok(Signature {
      consumer_id: consumer_id.to_owned(),
      keypair: keypair,
    })
  }

  /// Sign a request
  pub fn sign(&self, url: &str, method: Method, timestamp: i64) -> Result<String> {
    let input = format!("{consumer_id}\n{url}\n{method}\n{timestamp}\n",
      consumer_id = self.consumer_id,
      url = url,
      method = method,
      timestamp = timestamp
    );
    println!("sign: {}", input);
    
    let mut signer = Signer::new(MessageDigest::sha256(), &self.keypair)?;
    signer.update(input.as_bytes())?;
    let signature = signer.finish()?;

    Ok(encode(&signature))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dotenv::dotenv;
  use std::env;

  #[test]
  fn sign() {
    dotenv().ok();
    let signature = Signature::new(&env::var("WALMART_CONSUMER_ID").unwrap(), &env::var("WALMART_PRIVATE_KEY").unwrap()).unwrap();
    let signed = signature.sign("https://developer.walmart.com/proxy/item-api-doc-app/rest/v3/feeds?includeDetails=false&offset=0&limit=50", Method::Get, 1502165720641).unwrap();
    assert_eq!(signed, "LUgCoGaEHvJin/90eQFfcN5zvsy91geG86zFjHGdOkzs9VuPFTt5oZK2EHDRIt04bObc2T29fkoVVCJQuz29v99NK4bPechliZLigK/UuzgfMwU4xkY46/YZNt4IpZs8XepYY2Q3zg5zgLmrZnwpBPXpoQ8JktLzPGsokZWqe6g=");
  }
}