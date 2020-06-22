use crate::{constants, middlewares, routers};

use async_std::sync::RwLock;
use log::{self, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};
use std::env;

lazy_static! {
    // #[derive(Debug)]
    pub static ref RUST_ENV: String = init_env();
    pub static ref SETTINGS: RwLock<config::Config> = init_config();
}

pub fn init_with_state<State: Send + Sync + 'static>(
    state: State,
) -> tide::Server<State> {
    let mut app = tide::Server::with_state(state);
    app.at("/health").get(|_| async { Ok("") });

    middlewares::app_middleware(&mut app);
    routers::app_routers(&mut app);

    app
}

pub fn init() -> tide::Server<()> {
    init_with_state(())
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

pub async fn init_logger() {
    let requests_logger_path = SETTINGS
        .read()
        .await
        .get::<String>("requests_log")
        .unwrap_or("log/requests.log".to_owned());

    let mongo_logger_path = SETTINGS
        .read()
        .await
        .get::<String>("mongo_log")
        .unwrap_or("log/mongo.log".to_owned());

    let stdout = ConsoleAppender::builder()
        // .encoder(Box::new(PatternEncoder::new("{d} {m}{n}")))
        .build();

    let mongo = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {m}{n}")))
        .build(mongo_logger_path)
        .unwrap();

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
                .build(constants::LOG_APPENDER_MONGO, Box::new(mongo)),
        )
        .appender(
            Appender::builder()
                .build(constants::LOG_APPENDER_REQUESTS, Box::new(requests)),
        )
        .logger(
            Logger::builder()
                .appender(constants::LOG_APPENDER_MONGO)
                .additive(false)
                .build(
                    constants::LOG_TARGET_APP_BACKEND_MONGO,
                    LevelFilter::Info,
                ),
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
