use axum::http::StatusCode;
use tower::{BoxError, timeout::{self}};
use std::convert::Infallible;

pub fn handle_error(e: BoxError) -> Result<(StatusCode, String), Infallible> {
    if e.is::<timeout::error::Elapsed>() {
        Ok::<_, Infallible>((StatusCode::REQUEST_TIMEOUT, "request timeout".to_string()))
    } else {
        Ok::<_, Infallible>((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("unhandled error {}", e),
        ))
    }
}
