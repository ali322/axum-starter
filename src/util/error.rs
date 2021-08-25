use axum::{response::{IntoResponse, Json}, http::response::Response};
use serde_json::{Value, json};
use thiserror::Error;
use std::{num::ParseIntError, io::Error as IOError};
use validator::{ValidationErrors};
use crate::repository::DBError;

pub type APIResult = Result<Json<Value>, APIErrror>;

#[derive(Error, Debug)]
#[error("{}", .0)]
pub enum APIErrror {
    IO(#[from] IOError),
    Custom(&'static str),
    ParseInt(#[from] ParseIntError),
    Validate(#[from] ValidationErrors),
    DB(#[from] DBError)
}

impl IntoResponse for APIErrror {
    fn into_response(self) -> Response<hyper::Body> {
      let (code, message) = match self {
          _ => (-2, format!("{}", self)),
      };
        Json(json!({"code": code, "message": message})).into_response()
    }
}