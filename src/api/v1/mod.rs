use crate::util::Cors;
use axum::{routing::BoxRoute, Router};
use tower::layer::layer_fn;

mod auth;
mod post;
mod user;

pub fn apply_routes() -> Router<BoxRoute> {
    auth::apply_routes()
        .or(user::apply_routes())
        .or(post::apply_routes())
        .layer(layer_fn(|inner| Cors { inner }))
        .boxed()
}
