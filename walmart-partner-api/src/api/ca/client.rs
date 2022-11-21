use crate::result::WalmartResult;
use crate::WalmartCredential;
use reqwest::{RequestBuilder, Response};

pub struct Client {
  inner: crate::client::Client,
}

impl Client {
  pub fn new(credential: WalmartCredential) -> WalmartResult<Client> {
    let inner = crate::client::Client::new(crate::WalmartMarketplace::Canada, credential)?;
    Ok(Self { inner })
  }
}

impl std::ops::Deref for Client {
  type Target = crate::client::Client;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}
