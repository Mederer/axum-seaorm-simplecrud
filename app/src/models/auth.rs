use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{get_current_timestamp, DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

use crate::helpers::decode_token;

use super::errors::AuthError;

pub struct Keys {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

impl Keys {
    pub fn build() -> Keys {
        let token_key =
            dotenvy::var("TOKEN_KEY").expect("Error reading 'TOKEN_KEY' environment var");

        Self {
            encoding_key: EncodingKey::from_secret(token_key.as_bytes()),
            decoding_key: DecodingKey::from_secret(token_key.as_bytes()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
}

impl Claims {
    pub fn new(sub: &str) -> Self {
        Self {
            sub: sub.to_owned(),
            exp: (get_current_timestamp() + 60 * 60 * 24 * 3) as usize,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingToken)?;

        let claims = decode_token(bearer.token())?;

        Ok(claims)
    }
}
