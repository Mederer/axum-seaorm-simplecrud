use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{
    models::{errors::AppError, post::NewPost, AppState},
    services::post_service,
};

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(new_post): Json<NewPost>,
) -> Result<Json<Value>, AppError> {
    let new_post = post_service::create_post(&state.db, new_post).await?;

    Ok(Json(json!({
      "success": true,
      "new_post": new_post
    })))
}
