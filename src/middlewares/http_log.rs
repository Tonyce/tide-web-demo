use std::future::Future;
use std::pin::Pin;

use log;
use tide::{Middleware, Next, Request};
use uuid::Uuid;

#[derive(Debug)]
pub struct HttpLogMiddleware;

impl HttpLogMiddleware {
    pub fn new() -> Self {
        Self
    }

    /// Log a request and a response.
    async fn log<'a, State: Send + Sync + 'static>(
        &'a self,
        ctx: Request<State>,
        next: Next<'a, State>,
    ) -> tide::Result {
        let path = ctx.url().path().to_owned();
        let query = ctx.url().query().unwrap_or("");
        let method = ctx.method().to_string();

        let uuid: &Uuid = ctx.ext().unwrap();
        let uuid = uuid.to_string();

        log::info!(target: "app::requests", "<-- {} {} {} {}", uuid, method, path, query);

        let _start = std::time::Instant::now();
        let result: tide::Result = next.run(ctx).await;
        match result {
            Ok(res) => {
                let status = res.status();
                log::info!(target: "app::requests", "--> {} {}", uuid, status);
                Ok(res)
            }

            Err(err) => {
                log::info!(target: "app::requests", "--> {} {}", uuid, err.status());
                Err(err)
            }
        }
    }
}

impl<State: Send + Sync + 'static> Middleware<State> for HttpLogMiddleware {
    fn handle<'a>(
        &'a self,
        ctx: Request<State>,
        next: Next<'a, State>,
    ) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
        Box::pin(async move { self.log(ctx, next).await })
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
