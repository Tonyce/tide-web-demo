mod http_log;

pub use http_log::HttpLogMiddleware;

use uuid::Uuid;

pub fn app_middleware(app: &mut tide::Server<()>) {
    app.middleware(tide::Before(|mut request: tide::Request<()>| async move {
        // println!("before");
        request.set_ext(std::time::Instant::now());
        request.set_ext(Uuid::new_v4());
        request
    }));

    app.middleware(tide::After(|res: tide::Result| async move {
        // println!("after");
        let res = res.unwrap_or_else(|e| {
            println!("--- {:#?}", e);
            tide::Response::new(e.status())
        });
        let status = res.status();
        match status {
            tide::StatusCode::NotFound => Ok("Page not found".into()),
            tide::StatusCode::InternalServerError => Ok("Something went wrong".into()),
            tide::StatusCode::BadRequest => {
                println!("{:?}", status);
                Ok("...".into())
            }
            _ => Ok(res),
        }
    }));

    // filter middleware
    // app.middleware(HttpLogMiddleware::new());
}
