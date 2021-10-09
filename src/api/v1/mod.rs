use crate::util::Cors;
use axum::{routing::BoxRoute, Router};
use tower::layer::layer_fn;

mod auth;
mod post;
mod user;

pub fn apply_routes() -> Router<BoxRoute> {
    let mut v1 = Router::new().boxed();
    v1 = auth::apply_routes(v1.boxed());
    v1 = user::apply_routes(v1.boxed());
    v1 = post::apply_routes(v1.boxed());
    v1 = v1.layer(layer_fn(|inner| Cors { inner })).boxed();
    v1
}
