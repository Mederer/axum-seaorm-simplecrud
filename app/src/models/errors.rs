use axum::{http::StatusCode, response::IntoResponse};
use migration::DbErr;

pub enum AppError {
    AuthError(AuthError),
    DbError(DbErr),
    EntityNotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::AuthError(auth_error) => auth_error.into_response(),
            AppError::DbError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "A database error has occured.",
            )
                .into_response(),
            AppError::EntityNotFound => {
                (StatusCode::NOT_FOUND, "Entity not found.").into_response()
            }
        }
    }
}

impl From<DbErr> for AppError {
    fn from(value: DbErr) -> Self {
        AppError::DbError(value)
    }
}

impl From<AuthError> for AppError {
    fn from(value: AuthError) -> Self {
        Self::AuthError(value)
    }
}

pub enum AuthError {
    InvalidCredentials,
    Unauthorized,
    InvalidToken,
    TokenCreation,
    MissingToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials were given.").into_response()
            }
            AuthError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "You are not authorized to access this content.",
            )
                .into_response(),
            AuthError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Given token is invalid.").into_response()
            }
            AuthError::TokenCreation => (
                StatusCode::UNAUTHORIZED,
                "Error occured during token creation.",
            )
                .into_response(),
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token").into_response(),
        }
    }
}
