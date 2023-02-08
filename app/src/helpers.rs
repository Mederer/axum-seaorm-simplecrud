use jsonwebtoken::{decode, encode, Header, Validation};
use once_cell::sync::Lazy;

use crate::models::{
    auth::{Claims, Keys},
    errors::AuthError,
};

const TOKEN_KEYS: Lazy<Keys> = Lazy::new(|| Keys::build());

pub fn create_token(sub: &str) -> Result<String, String> {
    let claim = Claims {
        sub: sub.to_owned(),
        exp: 10000000000,
    };

    let token = encode(&Header::default(), &claim, &TOKEN_KEYS.encoding_key)
        .map_err(|_| String::from("Failed!"))?;

    Ok(token)
}

pub fn decode_token(token: String) -> Result<Claims, AuthError> {
    let decoded = decode::<Claims>(&token, &TOKEN_KEYS.decoding_key, &Validation::default())
        .map_err(|_| AuthError::InvalidToken)?;

    Ok(decoded.claims)
}