mod article;
mod hello;

use uuid::Uuid;

use crate::middlewares;

pub fn app_routers<State: Send + Sync + 'static>(
    app: &mut tide::Server<State>,
) {
    app.at("/api")
        .middleware(middlewares::HttpLogMiddleware::new())
        .nest({
            let mut api = tide::new();
            hello::hello_api(&mut api);
            article::article_api(&mut api);
            api
        });
    app.at("/ok").get(|req: tide::Request<_>| async move {
        let uuid: &Uuid = req.ext().unwrap();

        Ok(uuid.to_string())
    });
}
