use reqwest::StatusCode;
use std::error;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalmartError {
  #[error("{0}")]
  Msg(String),

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

  #[error("xml serialization error: {0}")]
  XmlSer(#[from] quick_xml::de::DeError),

  #[error("io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("zip error: {0}")]
  Zip(#[from] zip::result::ZipError),

  #[error("xml parse error: {0}")]
  XmlParse(#[from] xmltree::ParseError),

  #[error("csv error: {0}")]
  Csv(#[from] csv::Error),

  #[error("invalid header value: {0}")]
  InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

  #[error("unexpected xml: {0}")]
  UnexpectedXml(String),
}

impl From<String> for WalmartError {
  fn from(v: String) -> Self {
    Self::Msg(v)
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
      _ => false,
    }
  }
}

#[derive(Debug)]
pub struct ApiResponseError {
  pub message: String,
  pub status: StatusCode,
  pub body: String,
}

impl fmt::Display for ApiResponseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "API Error: status = '{}', message = '{}'",
      self.status, self.message
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
