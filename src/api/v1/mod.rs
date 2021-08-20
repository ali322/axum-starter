use std::{convert::Infallible, sync::Arc};

use crate::repository::DBPool;
use axum::{prelude::*, response::Json, routing::BoxRoute, AddExtensionLayer};
use hyper::StatusCode;
use serde_json::json;
use tower::ServiceBuilder;

mod auth;
mod user;

pub async fn index() -> &'static str {
    "hello world"
}

pub struct APIState {
    pub pool: DBPool,
}

pub fn apply_routes(pool: DBPool) -> BoxRoute<Body> {
    let prefix = "/api/v1";
    let api_state = Arc::new(APIState { pool });
    route(prefix, get(index))
        .route(
            format!("{}/register", prefix).as_str(),
            post(auth::register),
        )
        .route(format!("{}/login", prefix).as_str(), post(auth::login))
        .route(format!("{}/user/:id", prefix).as_str(), get(user::one))
        .route(format!("{}/user/:id", prefix).as_str(), put(user::update))
        .route(format!("{}/user", prefix).as_str(), get(user::all))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(api_state))
                //.layer(RequireAuthorizationLayer::custom(AuthRequest::new()))
                .into_inner(),
        )
        .handle_error(|err| {
            Ok::<_, Infallible>((
                StatusCode::OK,
                Json(json!({
                  "code": -1, "message": format!("{}", err)
                })),
            ))
        })
        .boxed()
}
