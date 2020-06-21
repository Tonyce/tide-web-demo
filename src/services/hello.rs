// #![feature(proc_macro_hygiene)]

use uuid::Uuid;

pub(crate) use crate::{controllers::hello::HelloQuery, models::Hello};

pub async fn say_hello(
    _uuid: &Uuid,
    _n: usize,
    _query: HelloQuery,
) -> Result<String, tide::Error> {
    let hello_word = Hello::say_hello();
    Ok(hello_word)
}

#[cfg(test)]
mod test {
    use mocktopus::mocking::*;

    use super::{say_hello, Hello, HelloQuery};
    use uuid::Uuid;

    #[async_std::test]
    async fn should_be_say_hello_ok() {
        Hello::say_hello.mock_safe(|| {
            mocktopus::mocking::MockResult::Return("mocking".to_owned())
        });

        let uuid = Uuid::new_v4();
        let n: usize = 9;
        let query = HelloQuery {
            word: "hello".to_owned(),
        };
        let result: String = say_hello(&uuid, n, query).await.unwrap();
        assert_eq!(result, "testHello");
    }
}
