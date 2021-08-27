use crate::util::{restrict::Restrict, todo::TodoMiddleware};
use axum::{
    handler::{get, post, Handler},
    routing::BoxRoute,
    Router,
};
use tower::{layer::layer_fn};
use tower_http::auth::RequireAuthorizationLayer;

mod auth;
mod user;

pub async fn index() -> &'static str {
    "hello world"
}

pub fn apply_routes() -> Router<BoxRoute> {
    let prefix = "/api/v1";
    let restrict_layer = RequireAuthorizationLayer::custom(Restrict::new());
    // let api_state = Arc::new(APIState { pool });
    let router = Router::new().route("/", get(index));
    let v1 = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/user", get(user::all.layer(layer_fn(|inner|TodoMiddleware{ inner }))))
        .route(
          "/user/:id",
          get(user::one.layer(restrict_layer.clone()))
          .put(user::update.layer(restrict_layer.clone())),
        );
    router.nest(prefix, v1.boxed()).boxed()
}
