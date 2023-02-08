use crate::models::{
    errors::AppError,
    post::{NewPost, Post},
};
use entity::post;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn get_posts_by_user_id(db: &DatabaseConnection, id: i32) -> Result<Vec<Post>, AppError> {
    let posts = post::Entity::find()
        .filter(post::Column::UserId.eq(id))
        .all(db)
        .await?;

    Ok(posts)
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
