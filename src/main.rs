use axum::prelude::RoutingDsl;

mod api;
mod lib;
mod repository;

#[tokio::main]
async fn main() {
    use axum::{Server};
    use dotenv::dotenv;
    use std::{net::SocketAddr, env};
    use api::v1::apply_routes;
    use repository::init_db_pool;

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "sqlx=INFO,app=DEBUG")
    }
    tracing_subscriber::fmt().pretty().init();
    dotenv().ok();
    let port = env::var("APP_PORT").expect("environment variable APP_PORT must be set");
    let port = port
        .parse::<u16>()
        .expect("environment variable APP_PORT must be u16");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let pool = init_db_pool().await;
    let routes = apply_routes(pool);
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("api started failed")
}
