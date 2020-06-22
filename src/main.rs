#![allow(unused_must_use)]
#![allow(dead_code)]
#![cfg_attr(test, feature(proc_macro_hygiene))]

#[macro_use]
extern crate lazy_static;

// use config::Config;

pub mod app;
mod constants;
mod controllers;
mod middlewares;
mod models;
mod routers;
mod services;

use log;

// use middlewares::app_middleware;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    app::init_logger().await;
    // log::info!("Settings: {:#?}", SETTINGS.read().await);

    // models::init_db().await.unwrap();
    // tide::log::start();
    let app = app::init();
    log::info!("ENV: {}, Server start at 8080", *app::RUST_ENV);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
