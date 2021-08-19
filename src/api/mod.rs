use axum::{body::Body, prelude::*, route, routing::BoxRoute};
use tower_http::auth::RequireAuthorizationLayer;

async fn index() -> &'static str {
    "hello world"
}

pub fn apply_routes() -> BoxRoute<Body> {
    route("/", get(index)).layer(crate::layer::restricted::RestrictedLayer::new()).boxed()
    // route("/", get(index))
    //     .layer(RequireAuthorizationLayer::custom(Restrict::new()))
    //     .boxed()
}
