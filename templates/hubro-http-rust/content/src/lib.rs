use std::collections::HashMap;
use std::convert::Into;
use anyhow::Result;
use spin_sdk::{http::{Request, Response, Router, Params, Method}, http_component, key_value::Store, redis_component};
use spin_sdk::http::IntoResponse;

#[http_component]
async fn entrypoint(req: Request) -> Response {
    match req.method() {
        Method::Get => {
            match req.path() {
                hubro_sdk::plugin::INFO_ENDPOINT => hubro_sdk::plugin::About::generate_info("test", "Test Plugin", "This is a sample Hubro plugin", "https://github.com/sample-account/hubro-plugin", "0.1.0", "", None).await.into_response(),
                _ => { Response::new(200, "") }
            }
        }
        Method::Other(_) => { Response::new(200, "") }
        _ => { Response::new(200, "") }
    }
}
