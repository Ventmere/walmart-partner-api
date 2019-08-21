use crate::result::*;
use crate::sign::Signature;
use chrono::Utc;
use rand::{thread_rng, Rng};
use reqwest;
pub use reqwest::{Method, Request, RequestBuilder, Url};

const BASE_URL: &'static str = "https://marketplace.walmartapis.com";

#[derive(Debug, Clone, Copy)]
pub enum WalmartMarketplace {
  USA,
  Canada,
}

pub trait ExtendUrlParams {
  fn extend_url_params(self, url: &mut Url);
}

impl ExtendUrlParams for () {
  fn extend_url_params(self, _: &mut Url) {}
}

impl<'a> ExtendUrlParams for &'a str {
  fn extend_url_params(self, url: &mut Url) {
    url.set_query(Some(self));
  }
}

impl<'a> ExtendUrlParams for String {
  fn extend_url_params(self, url: &mut Url) {
    if !self.is_empty() {
      url.set_query(Some(self.as_ref()));
    }
  }
}

impl<T1: AsRef<str>, T2: AsRef<str>> ExtendUrlParams for Vec<(T1, T2)> {
  fn extend_url_params(self, url: &mut Url) {
    url.query_pairs_mut().extend_pairs(self);
  }
}

pub struct Client {
  marketplace: WalmartMarketplace,
  channel_type: String,
  base_url: Url,
  sign: Signature,
  http: reqwest::Client,
}

impl Client {
  pub fn new(
    marketplace: WalmartMarketplace,
    channel_type: &str,
    consumer_id: &str,
    private_key: &str,
  ) -> WalmartResult<Client> {
    let http = reqwest::Client::new();
    Client::with_http_client(marketplace, channel_type, consumer_id, private_key, http)
  }

  pub fn with_http_client(
    marketplace: WalmartMarketplace,
    channel_type: &str,
    consumer_id: &str,
    private_key: &str,
    http: reqwest::Client,
  ) -> WalmartResult<Client> {
    Ok(Client {
      marketplace,
      channel_type: channel_type.to_string(),
      base_url: Url::parse(BASE_URL)?,
      sign: Signature::new(consumer_id, private_key)?,
      http: http,
    })
  }

  fn request<P>(&self, method: Method, path: &str, params: P) -> WalmartResult<RequestBuilder>
  where
    P: ExtendUrlParams,
  {
    use reqwest::header::HeaderMap;

    let mut url = match self.marketplace {
      WalmartMarketplace::USA => self.base_url.join(path)?,
      WalmartMarketplace::Canada => {
        // add `ca` to url
        let path = path
          .split('/')
          .enumerate()
          .map(|(i, seg)| {
            if i == 1 {
              format!("{}/ca", seg)
            } else {
              seg.to_string()
            }
          })
          .collect::<Vec<String>>()
          .join("/");
        self.base_url.join(&path)?
      }
    };
    params.extend_url_params(&mut url);

    let timestamp = Utc::now();
    let timestamp = timestamp.timestamp() * 1000 + timestamp.timestamp_subsec_millis() as i64;
    let sign = self.sign.sign(url.as_str(), method.clone(), timestamp)?;

    // println!("request: url = {}", url);
    // println!("request: timestamp = {}", timestamp);
    // println!("request: sign = {}", sign);

    let req = self.http.request(method.clone(), url);

    let mut headers = HeaderMap::new();
    let rid: String = thread_rng().gen_ascii_chars().take(10).collect();
    headers.insert("WM_SVC.NAME", "Walmart Marketplace".parse()?);
    headers.insert("WM_QOS.CORRELATION_ID", rid.parse()?);
    headers.insert("WM_SEC.TIMESTAMP", timestamp.to_string().parse()?);
    headers.insert("WM_SEC.AUTH_SIGNATURE", sign.parse()?);
    headers.insert("WM_CONSUMER.CHANNEL.TYPE", self.channel_type.parse()?);
    headers.insert("WM_CONSUMER.ID", self.sign.consumer_id().parse()?);
    let req = req.headers(headers);
    Ok(req)
  }

  pub fn request_json<P>(
    &self,
    method: Method,
    path: &str,
    params: P,
  ) -> WalmartResult<RequestBuilder>
  where
    P: ExtendUrlParams,
  {
    use reqwest::header::{HeaderValue, ACCEPT};

    self
      .request(method, path, params)
      .map(|req| req.header(ACCEPT, HeaderValue::from_static("application/json")))
  }

  pub fn request_xml<P>(
    &self,
    method: Method,
    path: &str,
    params: P,
  ) -> WalmartResult<RequestBuilder>
  where
    P: ExtendUrlParams,
  {
    use reqwest::header::{HeaderValue, ACCEPT};

    self
      .request(method, path, params)
      .map(|req| req.header(ACCEPT, HeaderValue::from_static("application/xml")))
  }

  pub(crate) fn get_marketplace(&self) -> WalmartMarketplace {
    self.marketplace
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use dotenv::dotenv;
//   use std::env;

//   #[test]
//   fn client() {
//     use std::io::Read;
//     dotenv().ok();

//     let client = Client::new(&env::var("WALMART_CONSUMER_ID").unwrap(), &env::var("WALMART_PRIVATE_KEY").unwrap()).unwrap();
//     let mut res = client.request_json(Method::Get, "/v3/feeds/117E39F0B7654B08A059457FB6E803FF@AQYBAAA", ()).unwrap().send().unwrap();
//     println!("status: {}", res.status());
//     let mut json = String::new();
//     res.read_to_string(&mut json).unwrap();
//     println!("body: {}", json);
//     {
//       use std::fs::File;
//       use std::io::Write;
//       let mut f = File::create("samples/get_feed_aand_item_status.json").unwrap();
//       write!(&mut f, "{}", json).unwrap()
//     }
//   }
// }
