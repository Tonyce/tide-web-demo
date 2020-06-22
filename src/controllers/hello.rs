// use core::future::Future;
// fn hello(_req: tide::Request<()>) -> impl Future<Output =
// tide::Result<String>> {     async_std::future::ready(Ok(String::from("hello"
// ))) }
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app::SETTINGS, services};

#[derive(Deserialize, Serialize)]
pub struct HelloQuery {
    pub word: String,
}

#[allow(dead_code)]
pub async fn get_goodbye(_req: tide::Request<()>) -> tide::Result<String> {
    // TODO: 参数校验
    // println!(
    //     "get_goodbye key: {}",
    //     SETTINGS
    //         .read()
    //         .await
    //         .get::<String>("key")
    //         .unwrap_or("".to_owned())
    // );
    Ok(String::from("goodbye"))
}

pub async fn get_hello(req: tide::Request<()>) -> tide::Result<String> {
    let uuid: &Uuid = req.ext().unwrap();
    let n: Result<usize, std::num::ParseIntError> = req.param("n");
    let query: HelloQuery = req.query().unwrap_or(HelloQuery {
        word: "".to_owned(),
    });

    // 参数验证
    let n = match n {
        Ok(num) => num,
        Err(_) => {
            let e = Err(tide::Error::from_str(
                tide::StatusCode::BadRequest,
                "参数错误".to_owned(),
            ));
            return e;
        }
    };

    let hello_word = services::hello::say_hello(uuid, n, query).await;
    // match hello_word {
    //     Ok(result) => Ok(format!("{}", result)),
    //     Err(e) => Err(e),
    // }
    hello_word
}

// #[cfg(test)]
// mod test {
//     use super::get_goodbye;
//     use tide::http::{Body, Method, Request, Response, Url};
//     #[async_std::test]
//     async fn should_be_get_goodbye() {
//         let req = Request::new(
//             Method::Get,
//             Url::parse("http://example.com/api/goodbye").unwrap(),
//         );
//         get_goodbye(req).await;
//     }
// }
