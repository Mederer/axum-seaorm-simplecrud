pub mod errors;

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
