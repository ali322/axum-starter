use crate::repository::DBError;
use axum::{
    body::Body,
    http::{response::Response, StatusCode},
    response::{IntoResponse, Json},
};
use serde_json::{json, Value};
use std::{io::Error as IOError, num::ParseIntError};
use thiserror::Error;
use validator::ValidationErrors;

pub type APIResult = Result<Json<Value>, APIErrror>;

#[derive(Error, Debug)]
#[error("{}", .0)]
pub enum APIErrror {
    IO(#[from] IOError),
    Custom(&'static str),
    ParseInt(#[from] ParseIntError),
    Validate(#[from] ValidationErrors),
    DBError(#[from] DBError),
}

impl IntoResponse for APIErrror {
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;
    fn into_response(self) -> Response<Body> {
        let (code, message) = match self {
            _ => (-2, format!("{}", self)),
        };
        let body = Body::from(json!({"code": code, "message": message}).to_string());
        Response::builder()
            .status(StatusCode::OK)
            .body(body)
            .unwrap()
    }
}
