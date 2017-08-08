error_chain! {
  foreign_links {
    Reqwest(::reqwest::Error);
    Base64(::base64::DecodeError);
    OpenSSL(::openssl::error::ErrorStack);
  }
}