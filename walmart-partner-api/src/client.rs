use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

use chrono::Utc;
use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
pub use reqwest::{Method, Request, RequestBuilder, Response, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::result::*;
use crate::sign::Signature;

const BASE_URL: &'static str = "https://marketplace.walmartapis.com";

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
}

impl Client {
  #[cfg(test)]
  pub fn set_base_url(&mut self, base_url: &str) {
    self.base_url = Url::parse(base_url).unwrap();
  }

  pub fn get_marketplace(&self) -> WalmartMarketplace {
    self.marketplace
  }

  pub fn req_json<P>(&self, method: Method, path: &str, params: P) -> WalmartResult<WalmartReq>
  where
    P: ExtendUrlParams,
  {
    use reqwest::header::ACCEPT;

    self
      .request(method, path, params)
      .map(|req| req.header(ACCEPT, HeaderValue::from_static("application/json")))
  }

  pub fn req_xml<P>(&self, method: Method, path: &str, params: P) -> WalmartResult<WalmartReq>
  where
    P: ExtendUrlParams,
  {
    use reqwest::header::ACCEPT;

    self
      .request(method, path, params)
      .map(|req| req.header(ACCEPT, HeaderValue::from_static("application/xml")))
  }

  pub async fn send(&self, req: WalmartReq) -> WalmartResult<WalmartRes> {
    let WalmartReq {
      url,
      method,
      rb: mut req,
    } = req;
    let timestamp = Utc::now();
    let timestamp = timestamp.timestamp() * 1000 + timestamp.timestamp_subsec_millis() as i64;

    let mut headers = HeaderMap::new();
    let rid = uuid::Uuid::new_v4().hyphenated().to_string();
    headers.insert("WM_SVC.NAME", "Walmart Marketplace".parse()?);
    headers.insert("WM_QOS.CORRELATION_ID", rid.parse()?);
    headers.insert("WM_SEC.TIMESTAMP", timestamp.to_string().parse()?);

    match self.auth_state {
      AuthState::Signature {
        ref channel_type,
        ref signature,
      } => {
        let sign = signature.sign(url.as_str(), method.clone(), timestamp)?;
        tracing::debug!("auth: Signature: sign = {}", sign);
        headers.insert("WM_CONSUMER.CHANNEL.TYPE", channel_type.parse()?);
        headers.insert("WM_CONSUMER.ID", signature.consumer_id().parse()?);
        headers.insert("WM_SEC.AUTH_SIGNATURE", sign.parse()?);
      }
      AuthState::TokenApi {
        ref client_id,
        ref client_secret,
        ..
      } => {
        let access_token = self.get_or_refresh_access_token(false).await?;
        tracing::debug!("auth: TokenApi: access_token = {}", access_token);
        headers.insert("WM_SEC.ACCESS_TOKEN", access_token.parse()?);
        req = req.basic_auth(client_id, Some(client_secret));
      }
    }
    let req = req.headers(headers);

    match req.send().await {
      Ok(res) => {
        if res.status() == StatusCode::UNAUTHORIZED {
          self.clear_access_token();
        }
        Ok(WalmartRes::new(res))
      }
      Err(err) => Err(err.into()),
    }
  }

  fn request<P>(&self, method: Method, path: &str, params: P) -> WalmartResult<WalmartReq>
  where
    P: ExtendUrlParams,
  {
    let mut url = match self.marketplace {
      WalmartMarketplace::USA => self.base_url.join(path)?,
      WalmartMarketplace::Canada => self.base_url.join(&path)?,
    };
    params.extend_url_params(&mut url);

    tracing::debug!("request: method = {}, url = {}", method, url);

    let req = self.http.request(method.clone(), url.as_str());

    Ok(WalmartReq::new(url, method, req))
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

  async fn get_or_refresh_access_token(&self, force_renew: bool) -> WalmartResult<String> {
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
            if token.expires_at.saturating_duration_since(Instant::now()) > Duration::from_secs(60)
            {
              return Ok(token.access_token.clone());
            }
          }
          drop(lock);
        }

        let mut form = HashMap::new();
        form.insert("grant_type", "client_credentials");

        let mut headers = HeaderMap::new();
        let rid = uuid::Uuid::new_v4().hyphenated().to_string();
        headers.insert("WM_SVC.NAME", "Walmart Marketplace".parse()?);
        headers.insert("WM_QOS.CORRELATION_ID", rid.parse()?);
        headers.insert("Accept", "application/json".parse()?);

        let res = self
          .http
          .request(Method::POST, &format!("{}/v3/token", BASE_URL))
          .headers(headers)
          .form(&form)
          .basic_auth(client_id, Some(client_secret))
          .send()
          .await?;

        let token = res.json::<WalmartBearerToken>().await?;
        let access_token = token.access_token.clone();

        if token.token_type != "Bearer" {
          return Err(WalmartError::Auth(format!(
            "unsupported token type: {}",
            token.token_type
          )));
        }

        tracing::debug!("token: {:#?}", token);
        tracing::debug!("token expires in {} seconds", token.expires_in);

        let mut lock = bearer_token.write().unwrap();
        lock.replace(BearerToken {
          access_token: token.access_token,
          expires_at: Instant::now() + Duration::from_secs(token.expires_in),
        });

        Ok(access_token)
      }
      _ => {
        return Err(WalmartError::Auth(
          "cannot get bearer with Signature Authentication".to_string(),
        ))
      }
    }
  }
}

pub struct WalmartReq {
  url: Url,
  method: Method,
  rb: RequestBuilder,
}

impl WalmartReq {
  pub fn new(url: Url, method: Method, rb: RequestBuilder) -> Self {
    Self { url, method, rb }
  }

  pub fn header(self, key: HeaderName, value: HeaderValue) -> Self {
    Self {
      rb: self.rb.header(key, value),
      ..self
    }
  }

  pub fn body_raw<R: std::io::Read + Send + 'static>(
    self,
    mut body: R,
    content_type: &'static str,
  ) -> WalmartResult<Self> {
    let mut buffer = Vec::new();
    body.read_to_end(&mut buffer)?;

    Ok(Self {
      rb: self.rb.body(buffer).header(
        reqwest::header::CONTENT_TYPE,
        HeaderValue::from_static(content_type),
      ),
      ..self
    })
  }

  pub fn body_json<T: Serialize>(self, body: &T) -> WalmartResult<Self> {
    Ok(Self {
      rb: self.rb.json(body),
      ..self
    })
  }

  /// Should switch to serde_xml_rs once they fix serialization issues
  /// some fields can't be serialized atm e.g. https://github.com/RReverser/serde-xml-rs/issues/186
  pub fn body_xml<T: crate::XmlSer>(self, body: T) -> WalmartResult<Self> {
    use xml_builder::{XMLBuilder, XMLVersion};
    let mut xml = XMLBuilder::new()
      .version(XMLVersion::XML1_0)
      .encoding("UTF-8".into())
      .build();
    xml.set_root_element(body.to_xml()?);
    let mut writer = Vec::<u8>::new();
    xml.generate(&mut writer)?;

    Ok(Self {
      rb: self
        .rb
        .header(reqwest::header::CONTENT_TYPE, "application/xml")
        .body(writer),
      ..self
    })
  }
}

impl Into<RequestBuilder> for WalmartReq {
  fn into(self) -> RequestBuilder {
    self.rb
  }
}

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
    if !self.is_empty() {
      url.set_query(Some(self));
    }
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

pub struct WalmartRes(Response);

impl WalmartRes {
  fn new(res: Response) -> Self {
    WalmartRes(res)
  }
}

impl WalmartRes {
  pub async fn res_bytes(self) -> WalmartResult<Vec<u8>> {
    let res = self.error_for_status().await?;
    let bytes = res.bytes().await?;
    Ok(bytes.to_vec())
  }

  pub async fn res_json<T: DeserializeOwned>(self) -> WalmartResult<T> {
    let res = self.error_for_status().await?;
    let json = res.json::<T>().await?;
    Ok(json)
  }

  pub async fn res_xml<T: DeserializeOwned>(self) -> WalmartResult<T> {
    let res = self.error_for_status().await?;

    let text = res.text().await?;
    serde_xml_rs::from_str(&text).map_err(|e| e.into())
  }

  async fn error_for_status(self) -> WalmartResult<Response> {
    let res = self.into_inner();
    if !res.status().is_success() {
      let status = res.status();
      let path = res.url().path().to_string();
      let body = res.text().await.unwrap_or_default();
      tracing::debug!(
        "walmart response error: status: '{}', path: '{}', response body: '{}'",
        status,
        path,
        body
      );
      Err(ApiResponseError { status, path, body }.into())
    } else {
      Ok(res)
    }
  }

  pub fn into_inner(self) -> Response {
    self.0
  }
}
