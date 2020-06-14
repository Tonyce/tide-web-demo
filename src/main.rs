mod middlewares;
mod routers;
mod services;

use log4rs;

// use middlewares::app_middleware;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    // tide::log::start();
    let mut app = tide::Server::new();
    app.at("/health").get(|_| async { Ok("") });

    middlewares::app_middleware(&mut app);
    routers::app_routers(&mut app);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
