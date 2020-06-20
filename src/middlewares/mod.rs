mod http_log;

pub use http_log::HttpLogMiddleware;

use uuid::Uuid;

pub fn app_middleware<State: Send + Sync + 'static>(app: &mut tide::Server<State>) {
    app.middleware(tide::Before(
        |mut request: tide::Request<State>| async move {
            // println!("before");
            request.set_ext(std::time::Instant::now());
            request.set_ext(Uuid::new_v4());
            request
        },
    ));

    app.middleware(tide::After(|res: tide::Result| async move {
        // println!("after");
        // let res = res.unwrap_or_else(|e| {
        //     println!("--- {:#?}", e);
        //     tide::Response::new(e.status())
        // });
        // let status = res.status();
        // match status {
        //     tide::StatusCode::NotFound => Err(res),
        //     tide::StatusCode::InternalServerError => Ok("Something went wrong".into()),
        //     _ => Ok(res),
        // }
        match res {
            Err(err) => {
                println!("err {:?}", err.status());
                Err(err)
            }
            Ok(r) => {
                let status = r.status();
                match status {
                    tide::StatusCode::NotFound => Err(tide::Error::from_str(
                        tide::StatusCode::NotFound,
                        "NotFound".to_owned(),
                    )),
                    tide::StatusCode::InternalServerError => Err(tide::Error::from_str(
                        tide::StatusCode::InternalServerError,
                        "InternalServerError".to_owned(),
                    )),
                    _ => Ok(r),
                }
            }
        }
    }));

    // filter middleware
    // app.middleware(HttpLogMiddleware::new());
}
