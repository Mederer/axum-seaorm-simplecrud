use entity::user;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set,
};

use crate::helpers::sub_and_id_match;
use crate::models::errors::{AppError, AuthError};
use crate::models::user::{NewUser, User, UserNoSecrets};

pub async fn get_user(db: &DatabaseConnection, id: i32) -> Result<User, AppError> {
    let user = user::Entity::find_by_id(id).one(db).await?;

    if let Some(user) = user {
        Ok(user)
    } else {
        Err(AppError::EntityNotFound)
    }
}

pub async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<User>, AppError> {
    let users = user::Entity::find().all(db).await?;

    Ok(users)
}

pub async fn update_user(
    db: &DatabaseConnection,
    updated_user: UserNoSecrets,
    sub: String,
) -> Result<User, AppError> {
    let user = user::Entity::find_by_id(updated_user.id).one(db).await?;

    let mut user: user::ActiveModel = if let Some(user) = user {
        if !sub_and_id_match(sub, &user) {
            return Err(AuthError::Unauthorized.into());
        };
        user.into()
    } else {
        return Err(AppError::EntityNotFound);
    };

    user.firstname = Set(updated_user.firstname);
    user.lastname = Set(updated_user.lastname);
    user.email = Set(updated_user.email);

    let user = user.update(db).await?;

    Ok(user)
}

pub async fn delete_user(db: &DatabaseConnection, id: i32) -> Result<(), AppError> {
    let user = user::Entity::find_by_id(id).one(db).await?;

    if let Some(user) = user {
        user.delete(db).await?;
        Ok(())
    } else {
        Err(AppError::EntityNotFound)
    }
}

pub async fn create_user(db: &DatabaseConnection, new_user: NewUser) -> Result<User, AppError> {
    if user::Entity::find()
        .filter(user::Column::Email.eq(new_user.email.clone()))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AuthError::EmailExists.into());
    }

    let new_user = user::ActiveModel {
        firstname: Set(new_user.firstname),
        lastname: Set(new_user.lastname),
        email: Set(new_user.email),
        secret: Set(new_user.secret),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(new_user)
}
