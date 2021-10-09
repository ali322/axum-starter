use chrono::{Local, NaiveDateTime, Duration};
use uuid::Uuid;

pub fn now() -> NaiveDateTime {
  Local::now().naive_local()
}

pub fn default_expire() -> NaiveDateTime {
  Local::now().naive_local() + Duration::days(30)
}

pub fn uuid_v4() -> String {
  Uuid::new_v4().to_string()
}