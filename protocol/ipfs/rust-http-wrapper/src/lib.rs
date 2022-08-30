pub mod wrap;
use ipfs_api_backend_actix::{IpfsClient, IpfsApi};
pub use wrap::*;
use std::fs::File;
use std::path::Path;
use core::str::Bytes;

use futures::Future;


#[actix_rt::main]
pub async fn add_file(args: ArgsAddFile) -> String {
  let client = IpfsClient::default();
  let path = std::str::from_utf8(&args.data).unwrap();
  match client.add_path(path).await {
    Ok(file) => String::new(),
    Err(e) => String::new(),
  }
}

#[actix_rt::main]
pub async fn cat(args: ArgsCat) -> Vec<u8> {
  vec![0]
}

#[actix_rt::main]
pub async fn resolve(args: ArgsResolve) -> std::option::Option<wrap::imported::ipfs_ipfs_resolve_result::IpfsIpfsResolveResult> {
  Some(IpfsIpfsResolveResult::new())
}
