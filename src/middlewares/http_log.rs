use std::future::Future;
use std::pin::Pin;
// use std::str::FromStr;

use log;
// use tide::http::Mime;
use tide::{Middleware, Next, Request};
use uuid::Uuid;

use crate::constants::LOG_TARGET_APP_REQUESTS;

#[derive(Debug)]
pub struct HttpLogMiddleware;

impl HttpLogMiddleware {
    pub fn new() -> Self {
        Self
    }

    /// Log a request and a response.
    async fn log<'a, State: Send + Sync + 'static>(
        &'a self,
        mut ctx: Request<State>,
        next: Next<'a, State>,
    ) -> tide::Result {
        // TODO: ?
        let body_str = if let Some(mime) = ctx.content_type() {
            let mut body_str = "".to_owned();
            if mime.essence() == "application/json" {
                body_str = ctx.body_string().await.unwrap_or("".to_string());
                ctx.set_body(body_str.clone());
            }
            body_str
        } else {
            "".to_owned()
        };

        let content_type = ctx.content_type();
        let content_len = ctx.len().unwrap_or(0);
        let path = ctx.url().path().to_owned();
        let query = ctx.url().query().unwrap_or("");
        let method = ctx.method().to_string();

        let uuid: &Uuid = ctx.ext().unwrap();
        let uuid = uuid.to_string();

        log::info!(
            target: LOG_TARGET_APP_REQUESTS,
            "<-- {} {} {} {} {:?} {} {}",
            uuid,
            path,
            query,
            method,
            content_type,
            content_len,
            body_str
        );

        let start = std::time::Instant::now();
        let result: tide::Result = next.run(ctx).await;

        let times_spend = start.elapsed().as_micros();
        match result {
            Ok(mut res) => {
                let status = res.status();

                // TODO: ?
                let body = res.take_body();
                let body_str =
                    body.into_string().await.unwrap_or("".to_owned());
                res.set_body(body_str.clone());

                log::info!(target: "app::requests", "--> {} {} {}ms {:?}", uuid, status, times_spend, body_str);

                Ok(res)
            }

            Err(err) => {
                log::info!(
                    target: LOG_TARGET_APP_REQUESTS,
                    "--> {} {}",
                    uuid,
                    err.status()
                );
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
