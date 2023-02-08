pub use entity::post::Model as Post;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub body: String,
}
