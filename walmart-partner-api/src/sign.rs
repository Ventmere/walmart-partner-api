//! Implement Walmart's authentication signature
//!
//! [Walmart Documentation](https://developer.walmart.com/#/apicenter/contentProvider#authentication)

use base64::encode;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Signer;
use reqwest::Method;

use result::*;

pub struct Signature {
  consumer_id: String,
  keypair: PKey,
}

impl Signature {
  /// Cosntruct a new `Signature`
  pub fn new(consumer_id: &str, private_key: &str) -> WalmartResult<Signature> {
    let pem_key = format!(
      "-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----",
      private_key
    );
    let keypair = Rsa::private_key_from_pem(pem_key.as_bytes())?;
    let keypair = PKey::from_rsa(keypair)?;
    Ok(Signature {
      consumer_id: consumer_id.to_owned(),
      keypair: keypair,
    })
  }

  /// Sign a request
  pub fn sign(&self, url: &str, method: Method, timestamp: i64) -> WalmartResult<String> {
    let input = format!(
      "{consumer_id}\n{url}\n{method}\n{timestamp}\n",
      consumer_id = self.consumer_id,
      url = url,
      method = method,
      timestamp = timestamp
    );

    let mut signer = Signer::new(MessageDigest::sha256(), &self.keypair)?;
    signer.update(input.as_bytes())?;
    let signature = signer.sign_to_vec()?;

    Ok(encode(&signature))
  }

  pub fn consumer_id(&self) -> &str {
    self.consumer_id.as_ref()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sign() {
    // Test data from https://github.com/fillup/walmart-auth-signature-php/blob/develop/tests/SignatureTest.php
    let fake_key = "MIICdgIBADANBgkqhkiG9w0BAQEFAASCAmAwggJcAgEAAoGBAKzXEfCYdnBNkKAwVbCpg/tR40WixoZtiuEviSEi4+LdnYAAPy57Qw6+9eqJGTh9iCB2wP/I8lWh5TZ49Hq/chjTCPeJiOqi6bvX1xzyBlSq2ElSY3iEVKeVoQG/5f9MYQLEj5/vfTWSNASsMwnNeBbbHcV1S1aY9tOsXCzRuxapAgMBAAECgYBjkM1j1OA9l2Ed9loWl8BQ8X5D6h4E6Gudhx2uugOe9904FGxRIW6iuvy869dchGv7j41ki+SV0dpRw+HKKCjYE6STKpe0YwIm/tml54aNDQ0vQvF8JWILca1a7v3Go6chf3Ib6JPs6KVsUuNo+Yd+jKR9GAKgnDeXS6NZlTBUAQJBANex815VAySumJ/n8xR+h/dZ2V5qGj6wu3Gsdw6eNYKQn3I8AGQw8N4yzDUoFnrQxqDmP3LOyr3/zgOMNTdszIECQQDNIxiZOVl3/Sjyxy9WHMk5qNfSf5iODynv1OlTG+eWao0Wj/NdfLb4pwxRsf4XZFZ1SQNkbNne7+tEO8FTG1YpAkAwNMY2g/ty3E6iFl3ea7UJlBwfnMkGz8rkye3F55f/+UCZcE2KFuIOVv4Kt03m3vg1h6AQkaUAN8acRl6yZ2+BAkEAke2eiRmYANiR8asqjGqr5x2qcm8ceiplXdwrI1kddQ5VUbCTonSewOIszEz/gWp6arLG/ADHOGWaCo8rptAyiQJACXd1ddXUAKs6x3l752tSH8dOde8nDBgF86NGvgUnBiAPPTmJHuhWrmOZmNaB68PsltEiiFwWByGFV+ld9VKmKg==";
    let signature = Signature::new("f3aead96-d681-41c9-9b81-bb4facacd8f0", fake_key).unwrap();
    let signed = signature.sign("https://developer.walmart.com/proxy/item-api-doc-app/rest/v3/feeds?includeDetails=false&offset=0&limit=50", Method::Get, 1502165720641).unwrap();
    assert_eq!(
      signed,
      "joVK3ddX6Fso7adAjuT1FIX5D5So8ue1Am4MwY8ncsP7zLBtnwMYiveyfQeqGm2+GQbtfOy5LvCkzUeEchLznJFZzF7vJaTHhENrDsRIzjPsgJYpRO8FgdfgSLUhO7v0skjHezMxuJr9ROWia900LOZ6QU+u/LvoChbxxZye9GE="
    );
  }
}
