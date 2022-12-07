use std::error;
use std::fmt;

use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalmartError {
  #[error("{0}")]
  Auth(String),

  #[error("{0}")]
  Csv(String),

  #[error("api response error: {0}")]
  Api(#[from] ApiResponseError),

  #[error("http error: {0}")]
  Reqwest(#[from] reqwest::Error),

  #[error("url parse error: {0}")]
  UrlParse(#[from] url::ParseError),

  #[error("base64 error: {0}")]
  Base64(#[from] base64::DecodeError),

  #[error("openssl error: {0}")]
  OpenSSL(#[from] openssl::error::ErrorStack),

  #[error("url query serialize error: {0}")]
  UrlEncoded(#[from] serde_urlencoded::ser::Error),

  #[error("json error: {0}")]
  JsonSerde(#[from] serde_json::Error),

  #[error("xml error: {0}")]
  XmlSerde(#[from] serde_xml_rs::Error),

  #[error("xml serialization error: {0}")]
  XmlSer(String),

  #[error("io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("{0}")]
  Zip(#[from] zip::result::ZipError),

  #[error("invalid header value: {0}")]
  InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}

impl From<xml_builder::XMLError> for WalmartError {
  fn from(e: xml_builder::XMLError) -> Self {
    Self::XmlSer(format!("{:?}", e))
  }
}

impl WalmartError {
  pub fn should_try_again(&self) -> bool {
    match *self {
      WalmartError::Reqwest(ref err) => {
        if let Some(status) = err.status() {
          let code = status.as_u16();
          // 429 Too Many Requests
          code == 429 || code == 500 || code == 503
        } else {
          false
        }
      }
      WalmartError::Api(ref err) => {
        let code = err.status.as_u16();
        code == 429 || code == 500 || code == 503
      }
      _ => false,
    }
  }
}

#[derive(Debug)]
pub struct ApiResponseError {
  pub path: String,
  pub status: StatusCode,
  pub body: String,
}

impl fmt::Display for ApiResponseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Walmart API error: status = '{}', path = '{}",
      self.status, self.path,
    )
  }
}

impl error::Error for ApiResponseError {
  fn description(&self) -> &str {
    "API response error"
  }

  fn cause(&self) -> Option<&dyn error::Error> {
    None
  }
}

pub type WalmartResult<T> = Result<T, WalmartError>;
