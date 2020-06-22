// tokio runtime use case

use tokio::time::delay_for;

use std::time::Duration;

pub fn tokio_runtime_eject() -> String {
    // smol::run(future: impl Future<Output = T>)
    smol::run(async {
        delay_for(Duration::from_micros(1_000)).await;
        "tokio_runtime_eject".to_owned()
    })
}
