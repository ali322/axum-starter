use crate::util::Cors;
use axum::Router;
use tower::layer::layer_fn;

mod auth;
mod post;
mod user;

pub fn apply_routes() -> Router {
    auth::apply_routes()
        .merge(user::apply_routes())
        .merge(post::apply_routes())
        .layer(layer_fn(|inner| Cors { inner }))
}
