use std::error;
use std::fmt;
use reqwest::StatusCode;

error_chain! {
  foreign_links {
    Reqwest(::reqwest::Error);
    UrlError(::reqwest::UrlError);
    Base64(::base64::DecodeError);
    OpenSSL(::openssl::error::ErrorStack);
    Api(ApiResponseError);
    QueryString(::serde_qs::Error);
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
    write!(f, "API Error: status = '{}', message = '{}'", self.status, self.message)
  }
}

impl error::Error for ApiResponseError {
  fn description(&self) -> &str {
    "API response error"
  }

  fn cause(&self) -> Option<&error::Error> {
    None
  }
}
