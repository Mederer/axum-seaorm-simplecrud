use crate::models::auth::Claims;
use crate::models::user::{NewUser, UserNoSecrets};
use crate::models::StateType;
use crate::{
    models::errors::AppError,
    services::{post_service, user_service},
};
use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value};

pub async fn get_user(
    State(state): StateType,
    _claims: Claims,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    let user = user_service::get_user(&state.db, id).await?;

    Ok(Json(json!({
        "success": true,
        "user": UserNoSecrets::from(user)
    })))
}

pub async fn create_user(
    State(state): StateType,
    _claims: Claims,
    Json(new_user): Json<NewUser>,
) -> Result<Json<Value>, AppError> {
    let new_user = user_service::create_user(&state.db, new_user).await?;

    Ok(Json(json!({
        "success": true,
        "new_user": UserNoSecrets::from(new_user)
    })))
}

pub async fn get_all_users(
    State(state): StateType,
    _claims: Claims,
) -> Result<Json<Value>, AppError> {
    let users = user_service::get_all_users(&state.db).await?;

    let users: Vec<UserNoSecrets> = users.into_iter().map(UserNoSecrets::from).collect();

    Ok(Json(json!({
        "success": true,
        "users": users
    })))
}

pub async fn update_user(
    State(state): StateType,
    _claims: Claims,
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
    State(state): StateType,
    Path(id): Path<i32>,
    _claims: Claims,
) -> Result<Json<Value>, AppError> {
    user_service::delete_user(&state.db, id).await?;

    Ok(Json(json!({
        "success": true,
        "message": format!("User {id} deleted.")
    })))
}

pub async fn get_posts(
    State(state): StateType,
    Path(id): Path<i32>,
    _claims: Claims,
) -> Result<Json<Value>, AppError> {
    let posts = post_service::get_posts_by_user_id(&state.db, id).await?;

    Ok(Json(json!({
        "success": true,
        "posts": posts,
    })))
}
