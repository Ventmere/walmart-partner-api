use std::fmt;

use chrono::{DateTime, TimeZone, Utc};
use serde::de;

/// Walmart serialize Date to a milliseconds since January 1, 1970 0:00:00 UTC,
/// It's not a standard unix timestamp, so we need to impl custom unserialize
struct TimestampVisitor;
impl<'de> de::Visitor<'de> for TimestampVisitor {
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
