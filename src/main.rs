use app::{api::apply_routes, util::handle_error};
use axum::Server;
use dotenv::dotenv;
use std::{env, net::SocketAddr, time::Duration};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::{compression::CompressionLayer};

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "app=DEBUG")
    }
    dotenv().ok();

    let log_path = std::env::var("LOG_PATH").expect("environment variable LOG_PATH must be set");
    let file_appender = tracing_appender::rolling::daily(log_path, "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .json()
        .with_writer(non_blocking)
        .init();

    let middlewares = ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        // .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());
    let routes = apply_routes().layer(middlewares).handle_error(handle_error);

    let port = env::var("APP_PORT").expect("environment variable APP_PORT must be set");
    let port = port
        .parse::<u16>()
        .expect("environment variable APP_PORT must be u16");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("app started failed")
}
