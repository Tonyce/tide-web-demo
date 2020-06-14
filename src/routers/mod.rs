pub mod hello;

use uuid::Uuid;

use crate::middlewares;

pub fn app_routers(app: &mut tide::Server<()>) {
    app.at("/api")
        .middleware(middlewares::HttpLogMiddleware::new())
        .nest({
            let mut api = tide::new();
            hello::hello_api(&mut api);
            api
        });
    app.at("/ok").get(|req: tide::Request<_>| async move {
        let uuid: &Uuid = req.ext().unwrap();

        Ok(uuid.to_string())
    });
}
