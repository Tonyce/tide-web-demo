use crate::services;

pub async fn new(_req: tide::Request<()>) -> tide::Result<String> {
    // TODO: 参数校验
    services::article::insert_article().await;
    Ok(String::from("goodbye"))
}
