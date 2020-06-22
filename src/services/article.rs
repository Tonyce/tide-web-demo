use crate::models;
use uuid::Uuid;

pub async fn insert_article(uuid: &Uuid) -> Result<String, tide::Error> {
    // model
    models::Article::insert_new(uuid, "title: &str", "author: &str").await;
    Ok("ok".to_owned())
}
