#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

pub mod app;
mod controllers;
mod middlewares;
mod models;
mod routers;
mod services;

use log4rs;

// use middlewares::app_middleware;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    // models::init_db().await.unwrap();
    // tide::log::start();
    let app = app::init();
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
