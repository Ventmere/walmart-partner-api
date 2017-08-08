use reqwest;
pub use reqwest::{Url, Method, Request, RequestBuilder};
use error::*;
use sign::Signature;
use chrono::Utc;
use rand::{thread_rng, Rng};

const BASE_URL: &'static str = "https://marketplace.walmartapis.com";
const CHANNEL_TYPE: &'static str = "0f3e4dd4-0514-4346-b39d-af0e00ea066d";

pub trait ExtendUrlParams {
  fn extend_url_params(self, url: &mut Url);
}

impl ExtendUrlParams for () {
  fn extend_url_params(self, url: &mut Url) {}
}

impl<'a> ExtendUrlParams for &'a str {
  fn extend_url_params(self, url: &mut Url) {
    url.set_query(Some(self));
  }
}

impl<T1: AsRef<str>, T2: AsRef<str>> ExtendUrlParams for Vec<(T1, T2)> {
  fn extend_url_params(self, url: &mut Url) {
    url.query_pairs_mut().extend_pairs(self);
  }
}

pub struct Client {
  base_url: Url,
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
      base_url: Url::parse(BASE_URL)?,
      sign: Signature::new(consumer_id, private_key)?,
      http: http,
    })
  }

  fn request<P>(&self, method: Method, path: &str, params: P) -> Result<RequestBuilder> 
    where P: ExtendUrlParams,
  {
    use reqwest::header::Headers;

    let mut url = self.base_url.join(path)?;
    params.extend_url_params(&mut url);

    let timestamp = Utc::now();
    let timestamp = timestamp.timestamp() * 1000 + timestamp.timestamp_subsec_millis() as i64;
    let sign = self.sign.sign(url.as_str(), method.clone(), timestamp)?;
    self.http.request(method.clone(), url)
      .map_err(Into::into)
      .map(|mut req| {
        let mut headers = Headers::new();
        let rid: String = thread_rng().gen_ascii_chars().take(10).collect();
        headers.set_raw("WM_SVC.NAME", "Walmart Gateway API");
        headers.set_raw("WM_QOS.CORRELATION_ID", rid.as_ref() as &str);
        headers.set_raw("WM_SEC.TIMESTAMP", timestamp.to_string().as_ref() as &str);
        headers.set_raw("WM_SEC.AUTH_SIGNATURE", sign.as_ref() as &str);
        headers.set_raw("WM_CONSUMER.CHANNEL.TYPE", CHANNEL_TYPE);
        headers.set_raw("WM_CONSUMER.ID", self.sign.consumer_id().as_ref() as &str);
        req.headers(headers);
        req
      })
  }

  pub fn request_json<P>(&self, method: Method, path: &str, params: P) -> Result<RequestBuilder> 
    where P: ExtendUrlParams,
  {
    use reqwest::header::{Accept, qitem};
    use reqwest::mime;
    
    self.request(method, path, params).map(|mut req| {
      req.header(Accept(vec![
        qitem(mime::APPLICATION_JSON)
      ]));
      req
    })
  }

  pub fn request_xml<P>(&self, method: Method, path: &str, params: P) -> Result<RequestBuilder> 
    where P: ExtendUrlParams,
  {
    use reqwest::header::{Accept, qitem};
    
    self.request(method, path, params).map(|mut req| {
      req.header(Accept(vec![
        qitem("application/xml".parse().unwrap())
      ]));
      req
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dotenv::dotenv;
  use std::env;

  // #[test]
  // fn name() {
  //   use std::io::Read;
  //   dotenv().ok();

  //   let client = Client::new(&env::var("WALMART_CONSUMER_ID").unwrap(), &env::var("WALMART_PRIVATE_KEY").unwrap()).unwrap();
  //   let mut res = client.request_json(Method::Get, "v3/orders", vec![
  //     ("createdStartDate", "2016-08-16T10:30:30.155Z"),
  //     ("status", "Acknowledged"),
  //   ]).unwrap().send().unwrap();
  //   println!("status: {}", res.status());
  //   let mut json = String::new();
  //   res.read_to_string(&mut json).unwrap();
  //   println!("body: {}", json);
  //   {
  //     use std::fs::File;
  //     use std::io::Write;
  //     let mut f = File::create("orders.json").unwrap();
  //     write!(&mut f, "{}", json).unwrap()
  //   }
  // }
}