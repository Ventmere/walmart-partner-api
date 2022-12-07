#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseError {
  #[serde(rename = "code")]
  pub code: String,
  #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
  pub field: Option<String>,
  #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
  pub info: Option<String>,
  #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
  pub severity: Option<String>,
  #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
  pub category: Option<String>,
  #[serde(rename = "causes", skip_serializing_if = "Option::is_none")]
  pub causes: Option<Vec<ResponseErrorCause>>,
  #[serde(rename = "errorIdentifiers", skip_serializing_if = "Option::is_none")]
  pub error_identifiers: Option<std::collections::HashMap<String, serde_json::Value>>,
  #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
  pub component: Option<String>,
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>,
  #[serde(rename = "serviceName", skip_serializing_if = "Option::is_none")]
  pub service_name: Option<String>,
  #[serde(
    rename = "gatewayErrorCategory",
    skip_serializing_if = "Option::is_none"
  )]
  pub gateway_error_category: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseErrorCause {
  #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
  pub code: Option<String>,
  #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
  pub field: Option<String>,
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>,
  #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}
