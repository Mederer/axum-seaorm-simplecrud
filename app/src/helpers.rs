use jsonwebtoken::{decode, encode, Header, Validation};
use once_cell::sync::Lazy;

use crate::models::{
    auth::{Claims, Keys},
    errors::AuthError,
    user::User,
};

static TOKEN_KEYS: Lazy<Keys> = Lazy::new(Keys::build);

pub fn create_token(sub: &str) -> Result<String, AuthError> {
    let claim = Claims::new(sub);

    let token = encode(&Header::default(), &claim, &TOKEN_KEYS.encoding_key)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(token)
}

pub fn decode_token(token: &str) -> Result<Claims, AuthError> {
    let decoded = decode::<Claims>(token, &TOKEN_KEYS.decoding_key, &Validation::default())
        .map_err(|_| AuthError::InvalidToken)?;

    Ok(decoded.claims)
}

pub fn sub_and_id_match(sub: String, user: &User) -> bool {
    let sub: i32 = sub.parse().expect(
        "Claims 'sub' field could not be parsed as an integer. This should not be possible.",
    );
    sub == user.id
}
