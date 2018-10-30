use chrono::{DateTime, TimeZone, Utc};
use serde::de;
use serde::Deserializer;
use std::fmt;

/// Walmart serialize Date to a milliseconds since January 1, 1970 0:00:00 UTC,
/// It's not a standard unix timestamp, so we need to impl custom unserialize
struct TimestampVistor;
impl<'de> de::Visitor<'de> for TimestampVistor {
  type Value = DateTime<Utc>;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a integer like `1502506180690`")
  }

  fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    Utc
      .timestamp_opt((v / 1000) as i64, ((v % 1000) * 1000000) as u32)
      .single()
      .ok_or_else(|| E::custom(format!("invalid timestamp value `{}`", v)))
  }
}

pub fn deserialize_timestamp<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
  D: Deserializer<'de>,
{
  d.deserialize_any(TimestampVistor)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_deserialize_timestamp() {
    use serde_json;

    #[derive(Deserialize)]
    struct T {
      #[serde(deserialize_with = "deserialize_timestamp")]
      date: DateTime<Utc>,
    }

    let t: T = serde_json::from_str(r##"{"date":1502506180690}"##).unwrap();
    assert_eq!(
      t.date.format("%Y-%m-%d").to_string(),
      "2017-08-12".to_string()
    );
  }
}
