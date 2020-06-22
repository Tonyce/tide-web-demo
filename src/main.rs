#![allow(unused_must_use)]
#![allow(dead_code)]
#![cfg_attr(test, feature(proc_macro_hygiene))]

#[macro_use]
extern crate lazy_static;

// use config::Config;
use async_std::sync::RwLock;
use std::env;

pub mod app;
mod constants;
mod controllers;
mod middlewares;
mod models;
mod routers;
mod services;

use log::{self, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};

// use middlewares::app_middleware;

lazy_static! {
    // #[derive(Debug)]
    static ref RUST_ENV: String = init_env();
    static ref SETTINGS: RwLock<config::Config> = init_config();
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    init_logger().await;
    // log::info!("Settings: {:#?}", SETTINGS.read().await);

    // models::init_db().await.unwrap();
    // tide::log::start();
    let app = app::init();
    log::info!("ENV: {}, Server start at 8080", *RUST_ENV);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn init_env() -> String {
    env::var("RUST_ENV").unwrap_or("dev".to_owned())
}

fn init_config() -> RwLock<config::Config> {
    RwLock::new({
        let mut settings = config::Config::default();
        settings
            .merge(config::File::with_name("Settings.toml"))
            .unwrap();

        settings
    })
}

async fn init_logger() {
    let requests_logger_path = SETTINGS
        .read()
        .await
        .get::<String>("requests_log")
        .unwrap_or("log/requests.log".to_owned());

    let stdout = ConsoleAppender::builder().build();
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {m}{n}")))
        .build(requests_logger_path)
        .unwrap();

    let config = Config::builder()
        .appender(
            Appender::builder()
                .build(constants::LOG_APPENDER_STDOUT, Box::new(stdout)),
        )
        .appender(
            Appender::builder()
                .build(constants::LOG_APPENDER_REQUESTS, Box::new(requests)),
        )
        .logger(
            Logger::builder()
                .build(constants::LOG_TARGET_APP_BACKEND_DB, LevelFilter::Info),
        )
        .logger(
            Logger::builder()
                .appender(constants::LOG_APPENDER_REQUESTS)
                .additive(false)
                .build(constants::LOG_TARGET_APP_REQUESTS, LevelFilter::Info),
        )
        .build(
            Root::builder()
                .appender(constants::LOG_APPENDER_STDOUT)
                .build(LevelFilter::Info),
        )
        .unwrap();
    log4rs::init_config(config).unwrap();
}
