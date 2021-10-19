#[tokio::main]
async fn main() {
    use axum::Server;
    use app::api::apply_routes;
    use dotenv::dotenv;
    use std::{env, net::SocketAddr};
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

    let port = env::var("APP_PORT").expect("environment variable APP_PORT must be set");
    let port = port
        .parse::<u16>()
        .expect("environment variable APP_PORT must be u16");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let routes = apply_routes();
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("app started failed")
}
