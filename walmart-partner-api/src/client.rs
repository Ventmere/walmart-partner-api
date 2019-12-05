use crate::result::*;
use crate::sign::Signature;
use chrono::Utc;
use rand::{thread_rng, Rng};
use reqwest;
use reqwest::header::HeaderMap;
pub use reqwest::{Method, Request, RequestBuilder, Response, StatusCode, Url};
use std::sync::RwLock;
use std::time::{Duration, Instant};

const BASE_URL: &'static str = "https://marketplace.walmartapis.com";

#[derive(Debug, Clone, Copy)]
pub enum WalmartMarketplace {
  USA,
  Canada,
}

pub enum WalmartCredential {
  TokenApi {
    client_id: String,
    client_secret: String,
  },
  Signature {
    channel_type: String,
    consumer_id: String,
    private_key: String,
  },
}

enum AuthState {
  TokenApi {
    client_id: String,
    client_secret: String,
    bearer_token: RwLock<Option<BearerToken>>,
  },
  Signature {
    channel_type: String,
    signature: Signature,
  },
}

struct BearerToken {
  access_token: String,
  expires_at: Instant,
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
  base_url: Url,
  auth_state: AuthState,
  http: reqwest::Client,
}

impl Client {
  pub fn new(
    marketplace: WalmartMarketplace,
    credential: WalmartCredential,
  ) -> WalmartResult<Client> {
    let http = reqwest::Client::new();
    Client::with_http_client(marketplace, credential, http)
  }

  pub fn with_http_client(
    marketplace: WalmartMarketplace,
    credential: WalmartCredential,
    http: reqwest::Client,
  ) -> WalmartResult<Client> {
    Ok(Client {
      marketplace,
      base_url: Url::parse(BASE_URL)?,
      auth_state: match credential {
        WalmartCredential::Signature {
          consumer_id,
          private_key,
          channel_type,
        } => AuthState::Signature {
          signature: Signature::new(&consumer_id, &private_key)?,
          channel_type,
        },
        WalmartCredential::TokenApi {
          client_id,
          client_secret,
        } => AuthState::TokenApi {
          client_id,
          client_secret,
          bearer_token: RwLock::new(None),
        },
      },
      http,
    })
  }

  fn request<P>(&self, method: Method, path: &str, params: P) -> WalmartResult<RequestBuilder>
  where
    P: ExtendUrlParams,
  {
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

    debug!("request: method = {}, url = {}", method, url);

    let timestamp = Utc::now();
    let timestamp = timestamp.timestamp() * 1000 + timestamp.timestamp_subsec_millis() as i64;

    let mut req = self.http.request(method.clone(), url.as_str());

    let mut headers = HeaderMap::new();
    let rid: String = thread_rng().gen_ascii_chars().take(10).collect();
    headers.insert("WM_SVC.NAME", "Walmart Marketplace".parse()?);
    headers.insert("WM_QOS.CORRELATION_ID", rid.parse()?);
    headers.insert("WM_SEC.TIMESTAMP", timestamp.to_string().parse()?);

    match self.auth_state {
      AuthState::Signature {
        ref channel_type,
        ref signature,
      } => {
        let sign = signature.sign(url.as_str(), method.clone(), timestamp)?;
        debug!("auth: Signature: sign = {}", sign);
        headers.insert("WM_CONSUMER.CHANNEL.TYPE", channel_type.parse()?);
        headers.insert("WM_CONSUMER.ID", signature.consumer_id().parse()?);
        headers.insert("WM_SEC.AUTH_SIGNATURE", sign.parse()?);
      }
      AuthState::TokenApi {
        ref client_id,
        ref client_secret,
        ..
      } => {
        let access_token = self.get_access_token(false)?;
        debug!("auth: TokenApi: access_token = {}", access_token);
        headers.insert("WM_SEC.ACCESS_TOKEN", access_token.parse()?);
        req = req.basic_auth(client_id, Some(client_secret));
      }
    }
    let req = req.headers(headers);
    Ok(req)
  }

  fn clear_access_token(&self) {
    match self.auth_state {
      AuthState::TokenApi {
        ref bearer_token, ..
      } => {
        bearer_token.write().unwrap().take();
      }
      _ => {}
    }
  }

  fn get_access_token(&self, force_renew: bool) -> WalmartResult<String> {
    use std::collections::HashMap;
    #[derive(Debug, Deserialize)]
    struct WalmartBearerToken {
      access_token: String,
      token_type: String,
      expires_in: u64,
    }

    match self.auth_state {
      AuthState::TokenApi {
        ref bearer_token,
        ref client_id,
        ref client_secret,
      } => {
        if !force_renew {
          let lock = bearer_token.read().unwrap();
          if let Some(ref token) = lock.as_ref() {
            if token.expires_at.saturating_duration_since(Instant::now()) > Duration::from_secs(120)
            {
              return Ok(token.access_token.clone());
            }
          }
          drop(lock);
        }

        let mut form = HashMap::new();
        form.insert("grant_type", "client_credentials");

        let mut headers = HeaderMap::new();
        let rid: String = thread_rng().gen_ascii_chars().take(10).collect();
        headers.insert("WM_SVC.NAME", "Walmart Marketplace".parse()?);
        headers.insert("WM_QOS.CORRELATION_ID", rid.parse()?);
        headers.insert("Accept", "application/json".parse()?);

        let mut res = self
          .http
          .request(Method::POST, &format!("{}/v3/token", BASE_URL))
          .headers(headers)
          .form(&form)
          .basic_auth(client_id, Some(client_secret))
          .send()?;

        let token: WalmartBearerToken = res.json()?;
        let access_token = token.access_token.clone();

        if token.token_type != "Bearer" {
          return Err(WalmartError::Msg(format!(
            "unsupported token type: {}",
            token.token_type
          )));
        }

        debug!("token: {:#?}", token);

        let mut lock = bearer_token.write().unwrap();
        std::mem::replace(
          &mut lock as &mut Option<_>,
          Some(BearerToken {
            access_token: token.access_token,
            expires_at: Instant::now() + Duration::from_secs(token.expires_in),
          }),
        );

        Ok(access_token)
      }
      _ => {
        return Err(WalmartError::Msg(
          "cannot get bearer with Signature Authentication".to_string(),
        ))
      }
    }
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

  pub fn send(&self, req: RequestBuilder) -> WalmartResult<Response> {
    match req.send() {
      Ok(res) => {
        if res.status() == StatusCode::UNAUTHORIZED {
          self.clear_access_token();
        }
        Ok(res)
      }
      Err(err) => Err(err.into()),
    }
  }

  pub fn get_marketplace(&self) -> WalmartMarketplace {
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
