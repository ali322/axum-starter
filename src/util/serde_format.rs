pub mod naive_datetime {
  use chrono::NaiveDateTime;
  use serde::{self, Deserialize, Deserializer, Serializer};

  const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

  pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      let s = format!("{}", date.format(FORMAT));
      serializer.collect_str(&s)
  }
  pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
  where
      D: Deserializer<'de>,
  {
      let s = String::deserialize(deserializer)?;
      NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
  }
}

pub mod utc_datetime {
  use chrono::{DateTime, TimeZone, Utc};
  use serde::{self, Deserialize, Deserializer, Serializer};

  pub fn serialize<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      let t = datetime.timestamp();
      serializer.serialize_i64(t)
  }
  pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
  where
      D: Deserializer<'de>,
  {
      Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
          .single()
          .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
  }
}

pub mod i32_bool {
  use serde::{self, Deserialize, Deserializer, Serializer};
  pub fn serialize<S>(val: &i32, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      let s = if *val == 1 { true } else { false };
      serializer.serialize_bool(s)
  }
  pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
  where
      D: Deserializer<'de>,
  {
      let d = i32::deserialize(deserializer)?;
      Ok(d)
  }
}
