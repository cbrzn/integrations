pub mod wrap;

use std::pin::Pin;
use futures::{FutureExt, TryFuture, Future};
pub use wrap::*;

/// Http Wrapper
///
/// Polywrap wrapper to execute Http requests.

type WrapperResponse = Pin<Box<dyn Future<Output=String>>>;

async fn execute_request(url: String) -> String {
    let res = reqwest::get(url).await.unwrap();
    res.text().await.unwrap()
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
