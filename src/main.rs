
mod api;
mod util;
mod repository;
mod entity;

#[tokio::main]
async fn main() {
    use api::v1::apply_routes;
    use axum::{prelude::*, Server};
    use dotenv::dotenv;
    use std::{env, net::SocketAddr};

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
    let conn = repository::init_db_conn().await;
    let routes = apply_routes(conn);
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("api started failed")
}
