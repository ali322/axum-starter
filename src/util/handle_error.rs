use axum::http::StatusCode;
use tower::{BoxError, timeout::{self}};

pub async fn handle_error(e: BoxError) -> (StatusCode, String) {
    if e.is::<timeout::error::Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "request timeout".to_string())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("unhandled error {}", e),
        )
    }
}
