use log;
use mongodb::bson::doc;
pub(crate) use uuid::Uuid;

use crate::{constants::LOG_TARGET_APP_BACKEND_MONGO, models::MONGO_DB};

pub struct Article {
    title: String,
    author: String,
}

impl Article {
    fn new(title: String, author: String) -> Self {
        Self { title, author }
    }

    fn collection() -> mongodb::Collection {
        MONGO_DB.collection("books")
    }

    // TODO: 包装 Error（自定义 Error）
    pub async fn insert_new(
        uuid: &Uuid,
        title: &str,
        author: &str,
    ) -> Result<String, mongodb::error::Error> {
        let collection = Self::collection();
        let doc = doc! {
            "title": title,
            "author": author,
        };

        let uuid = uuid.to_string();
        let start = std::time::Instant::now();
        log::info!(
            target: LOG_TARGET_APP_BACKEND_MONGO,
            "--> {} {}",
            uuid,
            doc
        );

        let result: mongodb::results::InsertOneResult = collection
            .insert_one(
                doc! {
                    "title": title,
                    "author": author,
                },
                None,
            )
            .await?;

        let times_spend = start.elapsed().as_millis();
        let inserted_id = result.inserted_id.as_object_id().unwrap().to_hex();
        log::info!(
            target: LOG_TARGET_APP_BACKEND_MONGO,
            "<-- {} {}ms {:?}",
            uuid,
            times_spend,
            inserted_id
        );

        Ok(inserted_id)
    }

    pub async fn find_article_with_id(id: &str) {
        let object_id = mongodb::bson::oid::ObjectId::with_string(id).unwrap();
        let collection = Self::collection();
        let document = collection
            .find_one(
                doc! {
                    "_id": object_id
                },
                None,
            )
            .await
            .unwrap()
            .unwrap();
        // if let Some(title) =
        //     document.get("title").and_then(mongodb::bson::Bson::as_str)
        // {
        //     println!("title: {}", title);
        // } else {
        //     println!("no title found");
        // }
        println!("title: {:?}", document.get("title"));
    }

    pub async fn update_article_with_id(id: &str, article: Article) {
        let object_id = mongodb::bson::oid::ObjectId::with_string(id).unwrap();
        let coll = Self::collection();

        let update_one_results = coll
            .update_one(
                doc! {
                    "_id": object_id
                },
                doc! {"$set": { "title": article.title }},
                None,
            )
            .await
            .unwrap();
        println!("{:?}", update_one_results);
        // assert_eq!(update_one_results.modified_count, 1);
        // assert!(update_one_results.upserted_id.is_none());
    }

    pub async fn delete_article_with_id(id: &str) {
        let object_id = mongodb::bson::oid::ObjectId::with_string(id).unwrap();
        let coll = Self::collection();
        let delete_one_result = coll
            .delete_one(doc! {"_id": object_id}, None)
            .await
            .unwrap();
        println!("{:?}", delete_one_result);
        // assert_eq!(delete_one_result.deleted_count, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::Uuid;
    use crate::models;

    #[async_std::test]
    // #[test]
    async fn should_insert_article() {
        let uuid = Uuid::new_v4();
        let insert_id =
            models::Article::insert_new(&uuid, "test-title", "test-author")
                .await
                .unwrap();
        println!("insert_id {}", insert_id);
        models::Article::delete_article_with_id(&insert_id).await;
        assert_eq!(insert_id.len(), 24);
    }

    #[async_std::test]
    async fn should_find_the_article() {
        let uuid = Uuid::new_v4();
        let insert_id =
            models::Article::insert_new(&uuid, "test-title", "test-author")
                .await
                .unwrap();
        models::Article::find_article_with_id(&insert_id).await;
        assert_eq!(insert_id.len(), 24);
    }

    #[async_std::test]
    async fn should_find_update_article() {
        let uuid = Uuid::new_v4();
        let insert_id =
            models::Article::insert_new(&uuid, "test-title", "test-author")
                .await
                .unwrap();
        models::Article::update_article_with_id(
            &insert_id,
            models::Article {
                title: "updatetitle".to_owned(),
                author: "".to_owned(),
            },
        )
        .await;
        assert_eq!(insert_id.len(), 24);
    }
}
