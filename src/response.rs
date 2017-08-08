use serde::de::DeserializeOwned;

#[derive(Debug, Deserialize)]
pub struct ListMeta {
  pub totalCount: i32,
  pub limit: i32,
  pub nextCursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListResponseInner<T: DeserializeOwned> {
  pub meta: ListMeta,
  pub element
}

#[derive(Debug, Deserialize)]
pub struct ListResponse<T: DeserializeOwned> {
  
}