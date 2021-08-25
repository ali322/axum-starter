macro_rules! reject {
    ($e: expr) => {
        crate::util::APIErrror::Custom($e)
    };
}

macro_rules! reply {
  ($t: tt) => {
    axum::response::Json(serde_json::json!({"code":0, "data": $t}))
  };
}

pub mod v1;