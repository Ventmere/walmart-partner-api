use reqwest::StatusCode;
use std::error;
use std::fmt;

error_chain! {
  foreign_links {
    Reqwest(::reqwest::Error);
    UrlError(::reqwest::UrlError);
    Base64(::base64::DecodeError);
    OpenSSL(::openssl::error::ErrorStack);
    Api(ApiResponseError);
    UrlEncoded(::serde_urlencoded::ser::Error);
    Io(::std::io::Error);
    Zip(::zip::result::ZipError);
    XmlParse(::xmltree::ParseError);
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

  fn cause(&self) -> Option<&error::Error> {
    None
  }
}
