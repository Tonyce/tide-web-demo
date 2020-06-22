use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};

use crate::controllers;

#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
}

pub fn hello_api(api: &mut tide::Server<()>) {
    api.at("/hello/:n").get(controllers::hello::get_hello);
    // api.at("/goodbye").get(|_| async { Ok("") });
    api.at("/goodbye").get(controllers::hello::get_goodbye);
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
