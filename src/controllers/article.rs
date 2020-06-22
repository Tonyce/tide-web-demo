use uuid::Uuid;

use crate::services;

pub async fn new(req: tide::Request<()>) -> tide::Result<String> {
    // TODO: 参数校验
    let uuid: &Uuid = req.ext().unwrap();
    services::article::insert_article(uuid).await;
    Ok(String::from("goodbye"))
}
