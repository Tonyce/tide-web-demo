#[cfg(test)]
mod hello {
    use async_std::task;
    use tide::http::{Body, Method, Request, Response, Url};
    use tide_web::app;

    #[async_std::test]
    async fn should_get_goodbye() {
        let app = app::init();
        // task::block_on( {
        let req = Request::new(
            Method::Get,
            Url::parse("http://example.com/api/goodbye").unwrap(),
        );
        let mut res: Response = app.respond(req).await.unwrap();

        let body_str = res.body_string().await.unwrap();
        // println!("{:#?}", body_str);
        assert_eq!(res.status(), 200);
        assert_eq!(body_str, "goodbye");
        // })
    }

    #[async_std::test]
    async fn should_post_hello_word() {
        let app = app::init();
        let body = "Hello, Nori!";
        let mut req = Request::new(
            Method::Post,
            Url::parse("http://example.com/api/goodbye").unwrap(),
        );
        req.set_body(body);

        let mut res: Response = app.respond(req).await.unwrap();
        let body_str = res.body_string().await.unwrap();
        println!("post_hello {:#?}", res.content_type());

        assert_eq!(res.status(), 200);
        assert_eq!(body_str, body);
    }

    #[async_std::test]
    async fn should_get_json() {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Debug)]
        struct Cat {
            name: String,
        }

        let app = app::init();
        let req = Request::new(
            Method::Get,
            Url::parse("http://example.com/api/json").unwrap(),
        );

        let mut res: Response = app.respond(req).await.unwrap();
        println!("get_json {:#?}", res.content_type());

        // let body_json = res.body_json::<Cat>().await.unwrap();
        let body_json: Cat = res.body_json().await.unwrap();
        println!("get_json {:#?}", body_json);

        assert_eq!(res.status(), 200);
        // assert_eq!(body_str, "goodbye");
    }

    #[async_std::test]
    async fn should_post_json() {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Debug)]
        struct Cat {
            name: String,
        }

        let app = app::init();
        let mut req = Request::new(
            Method::Post,
            Url::parse("http://example.com/api/json").unwrap(),
        );
        req.set_body(
            Body::from_json(&Cat {
                name: "chashu".into(),
            })
            .unwrap(),
        );

        let mut res: Response = app.respond(req).await.unwrap();
        println!("post_json {:#?}", res.content_type());

        // let body_json = res.body_json::<Cat>().await.unwrap();
        let body_json: Cat = res.body_json().await.unwrap();
        println!("post_json {:#?}", body_json);

        assert_eq!(res.status(), 200);
        // assert_eq!(body_str, "goodbye");
    }

    #[test]
    #[ignore]
    fn should_get_hello_word() {
        let app = app::init();
        task::block_on(async {
            let req = Request::new(
                Method::Get,
                Url::parse("http://example.com/api/hello/world").unwrap(),
            );
            let mut res: Response = app.respond(req).await.unwrap();

            let body_str = res.body_string().await.unwrap();
            // println!("{:#?}", body_str);
            assert_eq!(res.status(), 200);
            assert_eq!(body_str, "world");
        })
    }
}
