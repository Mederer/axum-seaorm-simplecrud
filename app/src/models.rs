pub mod errors;
pub mod post;
pub mod user;

use axum::extract::State;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub secret: String,
}

pub struct AppState {
    pub db: DatabaseConnection,
}

pub type StateType = State<Arc<AppState>>;
