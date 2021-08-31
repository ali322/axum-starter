use crate::util::datetime_format::utc_datetime;
use chrono::{DateTime, Duration, Timelike, Utc};
use jsonwebtoken::{
    decode, encode, errors::Result, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    #[serde(with = "utc_datetime")]
    pub iat: DateTime<Utc>,
    #[serde(with = "utc_datetime")]
    pub exp: DateTime<Utc>,
    pub auth: Auth,
}

impl Payload {
    pub fn new(auth: Auth, iat: DateTime<Utc>, exp: DateTime<Utc>) -> Self {
        let iat = iat
            .date()
            .and_hms_milli(iat.hour(), iat.minute(), iat.second(), 0);
        let exp = exp
            .date()
            .and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);
        Self { auth, iat, exp }
    }
}

pub fn generate_token<'a>(id: String, username: String) -> String {
    let iat = Utc::now();
    let exp = iat + Duration::days(30);
    let auth = Auth { id, username };
    let payload = Payload::new(auth, iat, exp);
    let jwt_key = env::var("JWT_KEY").expect("environment variable JWT_KEY must be set");
    encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(jwt_key.as_ref()),
    )
    .unwrap()
}

pub fn decode_token(token: &str) -> Result<TokenData<Payload>> {
    let jwt_key = env::var("JWT_KEY").expect("environment variable JWT_KEY must be set");
    decode::<Payload>(
        token,
        &DecodingKey::from_secret(jwt_key.as_ref()),
        &Validation::default(),
    )
}
