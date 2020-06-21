use crate::models;

pub async fn insert_article() -> Result<String, tide::Error> {
    // model
    models::Article::insert_new("title: &str", "author: &str").await;
    Ok("ok".to_owned())
}
