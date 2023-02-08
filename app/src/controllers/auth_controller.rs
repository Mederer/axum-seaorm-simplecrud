use crate::{
    models::{auth::Credentials, errors::AppError, StateType},
    services::auth_service,
};
use axum::{extract::State, Json};
use serde_json::{json, Value};

pub async fn authorize(
    State(state): StateType,
    Json(credentials): Json<Credentials>,
) -> Result<Json<Value>, AppError> {
    let token = auth_service::authorize(&state.db, credentials).await?;

    Ok(Json(json!({
        "success": true,
        "token": token
    })))
}
