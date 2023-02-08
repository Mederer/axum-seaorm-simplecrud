use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{
    models::{errors::AppError, post::NewPost, AppState, StateType},
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

pub async fn get_all_posts(State(state): StateType) -> Result<Json<Value>, AppError> {
    let posts = post_service::get_all_posts(&state.db).await?;

    Ok(Json(json!({
        "success": true,
        "posts": posts
    })))
}

pub async fn get_post() {
    todo!()
}

pub async fn delete_post() {
    todo!()
}

pub async fn update_post() {
    todo!()
}
