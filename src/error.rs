error_chain! {
  foreign_links {
    Reqwest(::reqwest::Error);
    UrlError(::reqwest::UrlError);
    Base64(::base64::DecodeError);
    OpenSSL(::openssl::error::ErrorStack);
  }
}