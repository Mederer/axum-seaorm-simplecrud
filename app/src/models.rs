use axum::{http::StatusCode, response::IntoResponse};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub secret: String,
}

pub struct AppState {
    pub db: DatabaseConnection,
}

pub enum AuthError {
    InvalidCredentials,
    Unauthorized,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthError::InvalidCredentials => (
                StatusCode::NOT_ACCEPTABLE,
                "Invalid credentials were given.",
            )
                .into_response(),
            AuthError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "You are not authorized to access this content.",
            )
                .into_response(),
        }
    }
}
