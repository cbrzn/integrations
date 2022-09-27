pub mod wrap;

use std::pin::Pin;
use futures::executor;
pub use wrap::*;

/// Http Wrapper
///
/// Polywrap wrapper to execute Http requests.

async fn execute_request(url: String) -> String {
    let res = reqwest::get(url).await.unwrap();
    res.text().await.unwrap()
}

pub fn get(args: ArgsGet) -> String {
    let result = executor::block_on(execute_request(args.url));
    
    String::from(result)
}
