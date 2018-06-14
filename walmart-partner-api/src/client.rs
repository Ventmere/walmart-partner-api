use chrono::Utc;
use error::*;
use rand::{thread_rng, Rng};
use reqwest;
pub use reqwest::{Method, Request, RequestBuilder, Url};
use sign::Signature;

const BASE_URL: &'static str = "https://marketplace.walmartapis.com";

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
  ) -> Result<Client> {
    let http = reqwest::Client::new();
    Client::with_http_client(marketplace, channel_type, consumer_id, private_key, http)
  }

  pub fn with_http_client(
    marketplace: WalmartMarketplace,
    channel_type: &str,
    consumer_id: &str,
    private_key: &str,
    http: reqwest::Client,
  ) -> Result<Client> {
    Ok(Client {
      marketplace,
      channel_type: channel_type.to_string(),
      base_url: Url::parse(BASE_URL)?,
      sign: Signature::new(consumer_id, private_key)?,
      http: http,
    })
  }

  fn request<P>(&self, method: Method, path: &str, params: P) -> Result<RequestBuilder>
  where
    P: ExtendUrlParams,
  {
    use reqwest::header::Headers;

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

    let mut req = self.http.request(method.clone(), url);

    let mut headers = Headers::new();
    let rid: String = thread_rng().gen_ascii_chars().take(10).collect();
    headers.set_raw("WM_SVC.NAME", "Walmart Marketplace");
    headers.set_raw("WM_QOS.CORRELATION_ID", rid.as_ref() as &str);
    headers.set_raw("WM_SEC.TIMESTAMP", timestamp.to_string().as_ref() as &str);
    headers.set_raw("WM_SEC.AUTH_SIGNATURE", sign.as_ref() as &str);
    headers.set_raw("WM_CONSUMER.CHANNEL.TYPE", &self.channel_type as &str);
    headers.set_raw("WM_CONSUMER.ID", self.sign.consumer_id().as_ref() as &str);
    req.headers(headers);
    Ok(req)
  }

  pub fn request_json<P>(&self, method: Method, path: &str, params: P) -> Result<RequestBuilder>
  where
    P: ExtendUrlParams,
  {
    use reqwest::header::{qitem, Accept};
    use reqwest::mime;

    self.request(method, path, params).map(|mut req| {
      req.header(Accept(vec![qitem(mime::APPLICATION_JSON)]));
      req
    })
  }

  pub fn request_xml<P>(&self, method: Method, path: &str, params: P) -> Result<RequestBuilder>
  where
    P: ExtendUrlParams,
  {
    use reqwest::header::{qitem, Accept};

    self.request(method, path, params).map(|mut req| {
      req.header(Accept(vec![qitem("application/xml".parse().unwrap())]));
      req
    })
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
