// tokio runtime use case

use tokio::time::delay_for;

use std::time::Duration;

pub async fn tokio_runtime_eject() -> String {
    // smol::run(future: impl Future<Output = T>)
    // smol::run(async {
    //     delay_for(Duration::from_micros(1_000)).await;
    //     "tokio_runtime_eject".to_owned()
    // })
    // smol::Task::blocking(future: impl Future<Output = T> + Send + 'static)
    smol::Task::blocking(async {
        delay_for(Duration::from_micros(1_000)).await;
        "tokio_runtime_eject".to_owned()
    })
    .await
}
