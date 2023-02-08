use crate::{
    models::{errors::AppError, AppState},
    services::user_service,
};
use axum::{
    extract::{Path, State},
    Json,
};
use entity::user::{NewUser, UserNoSecrets};
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    let user = user_service::get_user(&state.db, id).await?;

    Ok(Json(json!({
        "success": true,
        "user": UserNoSecrets::from(user)
    })))
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<Value>, AppError> {
    let new_user = user_service::create_user(&state.db, new_user).await?;

    Ok(Json(json!({
        "success": true,
        "new_user": UserNoSecrets::from(new_user)
    })))
}

pub async fn get_all_users(State(state): State<Arc<AppState>>) -> Result<Json<Value>, AppError> {
    let users = user_service::get_all_users(&state.db).await?;

    let users: Vec<UserNoSecrets> = users.into_iter().map(UserNoSecrets::from).collect();

    Ok(Json(json!({
        "success": true,
        "users": users
    })))
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Json(updated_user): Json<UserNoSecrets>,
) -> Result<Json<Value>, AppError> {
    let updated_user: UserNoSecrets = user_service::update_user(&state.db, updated_user)
        .await?
        .into();

    Ok(Json(json!({
        "success": true,
        "updated_user": updated_user
    })))
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    user_service::delete_user(&state.db, id).await?;

    Ok(Json(json!({
        "success": true,
        "message": format!("User {id} deleted.")
    })))
}
