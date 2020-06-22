use mongodb::{bson::doc, options::ClientOptions, Client};

mod article;
pub(crate) use article::Article;

mod hello;
pub(crate) use hello::Hello;

use crate::app::SETTINGS;

lazy_static! {
    pub static ref MONGO_DB: mongodb::Database = init_db();
}

fn init_db() -> mongodb::Database {
    // println!("init_db");

    async_std::task::block_on(async {
        let mongodb_url = SETTINGS
            .read()
            .await
            .get::<&str>("mongodb")
            .unwrap_or("mongodb://localhost:27017");

        let mut client_options =
            ClientOptions::parse(mongodb_url).await.unwrap();

        client_options.app_name = Some("My App".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options).unwrap();

        let db = client.database("mydb");
        // let collection = db.collection("books");
        db
    })
}

// pub fn init_db() -> mongodb::Database {
//     let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
//     let database = client.database("mydb");
//     database
// MONGO_DB = Some(db);
// let collection = db.collection("books");

// let docs = vec![
//     doc! { "title": "1984", "author": "George Orwell" },
//     doc! { "title": "Animal Farm", "author": "George Orwell" },
//     doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
// ];

// // Insert some documents into the "mydb.books" collection.
// collection.insert_many(docs, None).await?;
// Ok(())
// }
