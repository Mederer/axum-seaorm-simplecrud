use migration::Condition;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    helpers::create_token,
    models::{
        auth::Credentials,
        errors::{AppError, AuthError},
    },
};
use entity::user;

pub async fn authorize(
    db: &DatabaseConnection,
    credentials: Credentials,
) -> Result<String, AppError> {
    let user = user::Entity::find()
        .filter(
            Condition::all()
                .add(user::Column::Email.eq(credentials.email))
                .add(user::Column::Secret.eq(credentials.secret)),
        )
        .one(db)
        .await?;

    if let Some(user) = user {
        let token =
            create_token(user.id.to_string().as_str()).map_err(|_| AuthError::TokenCreation)?;
        Ok(token)
    } else {
        Err(AuthError::InvalidCredentials.into())
    }
}
