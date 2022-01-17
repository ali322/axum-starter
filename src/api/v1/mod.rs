use std::collections::HashMap;

use crate::middleware::{Cors, JWT};
use axum::Router;
use tower::layer::layer_fn;
use tower_http::auth::RequireAuthorizationLayer;

mod auth;
mod post;
mod user;

pub fn apply_routes() -> Router {
  let mut unless = HashMap::new();
  unless.insert(r"^/public".to_string(), "get|post".to_string());
  let restrict_layer = RequireAuthorizationLayer::custom(JWT::new(unless));
    auth::apply_routes()
        .merge(user::apply_routes())
        .merge(post::apply_routes())
        .layer(restrict_layer)
        .layer(layer_fn(|inner| Cors { inner }))
}
