use axum::{Router, handler::get, routing::BoxRoute};

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

mod v1;

async fn index() -> &'static str {
  "hello world"
}

pub fn apply_routes() -> Router<BoxRoute> {
  let prefix = "/api/v1";
  let router = Router::new().route("/", get(index));
  router.nest(prefix, v1::apply_routes()).boxed()
}