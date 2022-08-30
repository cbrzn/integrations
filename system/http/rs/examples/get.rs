use std::pin::Pin;
use futures::{FutureExt, TryFuture, Future};

pub struct ArgsGet {
    pub url: String,
}

async fn execute_request(url: String) -> String {
    let res = reqwest::get(url).await.unwrap();
    res.text().await.unwrap()
}

pub struct Response {
    pub status: i32,
    pub data: String,
}

fn execute_request_wrapper<T>(url: String) -> Option<String> {
    execute_request(url).now_or_never()
}

pub fn get(args: ArgsGet) -> Response {
    match execute_request_wrapper::<String>(args.url) {
        Some(data) => {
            Response {
                status: 200,
                data,
            }
        }
        _ => {
            Response {
                status: 200,
                data: String::new(),
            }
        }
    }
}

fn main() {
    get(ArgsGet { url: String::from("https://github.com") });
}
