use crate::result::WalmartResult;
use crate::WalmartCredential;

pub struct Client {
  inner: crate::client::Client,
}

impl Client {
  pub fn new(credential: WalmartCredential) -> WalmartResult<Self> {
    let inner = crate::client::Client::new(crate::WalmartMarketplace::USA, credential)?;
    Ok(Self { inner })
  }
}

impl std::ops::Deref for Client {
  type Target = crate::client::Client;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl std::ops::DerefMut for Client {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}
