use reqwest::StatusCode;
use std::error;
use std::fmt;

#[derive(Fail, Debug)]
pub enum WalmartError {
  #[fail(display = "{}", _0)]
  Msg(String),

  #[fail(display = "api response error: {}", _0)]
  Api(ApiResponseError),

  #[fail(display = "http error: {}", _0)]
  Reqwest(::reqwest::Error),

  #[fail(display = "url error: {}", _0)]
  UrlError(::reqwest::UrlError),

  #[fail(display = "base64 error: {}", _0)]
  Base64(::base64::DecodeError),

  #[fail(display = "openssl error: {}", _0)]
  OpenSSL(::openssl::error::ErrorStack),

  #[fail(display = "url query serialize error: {}", _0)]
  UrlEncoded(::serde_urlencoded::ser::Error),

  #[fail(display = "io error: {}", _0)]
  Io(::std::io::Error),

  #[fail(display = "zip error: {}", _0)]
  Zip(::zip::result::ZipError),

  #[fail(display = "xml parse error: {}", _0)]
  XmlParse(::xmltree::ParseError),

  #[fail(display = "csv error: {}", _0)]
  Csv(::csv::Error),
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

macro_rules! impl_from {
  ($v:ident($t:ty)) => {
    impl From<$t> for WalmartError {
      fn from(e: $t) -> Self {
        WalmartError::$v(e)
      }
    }
  };
}

impl_from!(Msg(String));
impl_from!(Api(ApiResponseError));
impl_from!(Reqwest(::reqwest::Error));
impl_from!(UrlError(::reqwest::UrlError));
impl_from!(Base64(::base64::DecodeError));
impl_from!(OpenSSL(::openssl::error::ErrorStack));
impl_from!(UrlEncoded(::serde_urlencoded::ser::Error));
impl_from!(Io(::std::io::Error));
impl_from!(Zip(::zip::result::ZipError));
impl_from!(XmlParse(::xmltree::ParseError));
impl_from!(Csv(::csv::Error));

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

pub type WalmartResult<T> = Result<T, WalmartError>;
