#[cfg(test)]
mod health {
    use async_std::task;
    use tide::http::{Method, Request, Response, Url};
    use tide_web::app;

    #[test]
    fn should_get_health() {
        let app = app::init();
        task::block_on(async {
            let req = Request::new(
                Method::Get,
                Url::parse("http://example.com/health").expect("url ok"),
            );
            let res: Response = app.respond(req).await.unwrap();

            assert_eq!(res.status(), 200);
        })
    }
}
