use crate::models::{
    errors::AppError,
    post::{NewPost, Post},
};
use entity::{post, user};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn get_posts_by_user_id(db: &DatabaseConnection, id: i32) -> Result<Vec<Post>, AppError> {
    let user = user::Entity::find_by_id(id).one(db).await?;

    if let Some(user) = user {
        let posts = post::Entity::find()
            .filter(post::Column::UserId.eq(user.id))
            .all(db)
            .await?;

        Ok(posts)
    } else {
        Err(AppError::EntityNotFound)
    }
}

pub async fn create_post(db: &DatabaseConnection, new_post: NewPost) -> Result<Post, AppError> {
    let new_post = post::ActiveModel {
        title: Set(new_post.title),
        body: Set(new_post.body),
        user_id: Set(new_post.user_id),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(new_post)
}

pub async fn get_all_posts(db: &DatabaseConnection) -> Result<Vec<Post>, AppError> {
    let posts = post::Entity::find().all(db).await?;

    Ok(posts)
}
