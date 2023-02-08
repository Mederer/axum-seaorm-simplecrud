use serde::Deserialize;

pub use entity::post::Model as Post;

#[derive(Deserialize)]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub body: String,
}
