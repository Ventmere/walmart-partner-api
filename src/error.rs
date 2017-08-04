use reqwest;

error_chain! {
  foreign_links {
    Reqwest(reqwest::Error);
  }
}