use axum::{
    routing::BoxRoute,
    Router,
};

mod user;
mod auth;
mod post;

pub fn apply_routes() -> Router<BoxRoute> {
    let mut v1 = Router::new().boxed();
    v1 = auth::apply_routes(v1.boxed());
    v1 = user::apply_routes(v1.boxed());
    v1 = post::apply_routes(v1.boxed());
    v1
}
