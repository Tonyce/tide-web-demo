use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};
use uuid::Uuid;

use crate::services;

#[derive(Deserialize, Serialize)]
pub struct HelloQuery {
    word: String,
}

#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
}

pub fn hello_api(api: &mut tide::Server<()>) {
    api.at("/hello/:n").get(|req: tide::Request<_>| async move {
        let uuid: Option<&Uuid> = req.ext();
        let n: Result<usize, std::num::ParseIntError> = req.param("n");
        let query: Result<HelloQuery, tide::Error> = req.query();

        let hello_word = services::hello::say_hello(uuid, n, query).await;
        // match hello_word {
        //     Ok(result) => Ok(format!("{}", result)),
        //     Err(e) => Err(e),
        // }
        hello_word
    });
    api.at("/goodbye").get(|_| async { Ok("") });
    api.at("/goodbye").post(|mut req: Request<_>| async move {
        let body: String = req.body_string().await.unwrap();
        Ok(body)
    });

    api.at("/json").get(|_| async {
        let cat = Cat {
            name: "chashu".into(),
        };
        let mut res = Response::new(200);
        res.set_body(Body::from_json(&cat)?);
        Ok(res)
    });

    api.at("/json").post(|mut req: Request<()>| async move {
        println!("api post_json {:#?}", req.content_type());
        let cat: Cat = req.body_json().await?;
        println!("cat name: {}", cat.name);

        let cat = Cat {
            name: "chashu".into(),
        };

        let mut res = Response::new(200);
        res.set_body(Body::from_json(&cat)?);
        Ok(res)
    });
}
