use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services;

#[derive(Deserialize, Serialize)]
pub struct HelloQuery {
    word: String,
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
    api.at("/goodbye").post(|_| async { Ok("Goodbye, world") });
}
