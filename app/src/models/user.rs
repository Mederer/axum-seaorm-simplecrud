use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

pub use entity::user::Model as User;

#[derive(Serialize, FromQueryResult, Deserialize)]
pub struct UserNoSecrets {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub secret: String,
}

impl NewUser {
    pub fn new(firstname: &str, lastname: &str, email: &str, secret: &str) -> Self {
        Self {
            firstname: firstname.to_owned(),
            lastname: lastname.to_owned(),
            email: email.to_owned(),
            secret: secret.to_owned(),
        }
    }
}

impl From<User> for UserNoSecrets {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            firstname: value.firstname,
            lastname: value.lastname,
            email: value.email,
        }
    }
}
