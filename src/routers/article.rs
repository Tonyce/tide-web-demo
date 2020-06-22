use serde::{Deserialize, Serialize};
use tide::{Body, Response};

use crate::controllers;

#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
}

pub fn article_api(api: &mut tide::Server<()>) {
    api.at("/article").get(|_| async {
        let cat = Cat {
            name: "chashu".into(),
        };
        let mut res = Response::new(200);
        res.set_body(Body::from_json(&cat)?);
        Ok(res)
    });

    api.at("/article").post(controllers::article::new);
}
