use entity::user;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn get_user(db: &DatabaseConnection, id: i32) -> Option<user::Model> {
    let user = user::Entity::find_by_id(id)
        .one(db)
        .await
        .expect("A database error has occured!");

    user
}

pub async fn get_all_users(db: &DatabaseConnection) -> Option<Vec<user::Model>> {
    let users = user::Entity::find().all(db).await;

    if let Ok(users) = users {
        return Some(users);
    } else {
        return None;
    }
}
