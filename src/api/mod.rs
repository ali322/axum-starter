use axum::{body::Body, prelude::*, route, routing::BoxRoute};

async fn index() -> &'static str {
    "hello world"
}

pub fn apply_routes() -> BoxRoute<Body> {
    route("/", get(index))
        .layer(crate::layer::restricted::RestrictedLayer::new())
        .boxed()
}
