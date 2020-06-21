use uuid::Uuid;

use crate::controllers::hello::HelloQuery;

pub async fn say_hello(_uuid: &Uuid, _n: usize, _query: HelloQuery) -> Result<String, tide::Error> {
    // model
    Ok("say hello".to_owned())
}
