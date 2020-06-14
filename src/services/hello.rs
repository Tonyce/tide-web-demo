use uuid::Uuid;

use crate::routers::hello::HelloQuery;

pub async fn say_hello(
    _uuid: Option<&Uuid>,
    n: Result<usize, std::num::ParseIntError>,
    _query: Result<HelloQuery, http_types::Error>,
) -> Result<String, tide::Error> {
    // 参数验证
    let _num = match n {
        Ok(num) => num,
        Err(_) => {
            let e = Err(tide::Error::from_str(
                tide::StatusCode::BadRequest,
                "参数错误".to_owned(),
            ));
            return e;
        }
    };
    Ok("say hello".to_owned())
}
