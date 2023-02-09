pub mod auth;
pub mod errors;
pub mod post;
pub mod user;

use axum::extract::State;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub type StateType = State<Arc<AppState>>;
