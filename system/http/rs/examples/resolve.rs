use ipfs_api_backend_hyper::{IpfsClient, IpfsApi};
use std::fs::File;
use std::path::Path;
use futures::TryStreamExt;

struct Status {
  ok: bool,
  error: String
}
#[tokio::main]
async fn main() -> Result<(), &'static str> {
  let client = IpfsClient::default();
  let file = String::from("QmPvBTYmUfeJoVSbt5h1ECyR2eQ9DLZMAM8Tqu4JkKkP7o");
  let cool = client
    .cat(&file[..])
    .map_ok(|chunk| chunk.to_vec())
    .try_concat()
    .await
    .map_err(|_| "error reading full file")?;

  eprintln!("---------------------------------------");
  eprintln!("{}", String::from_utf8_lossy(&cool[..]));
  eprintln!("---------------------------------------");
  Ok(())

}


impl Status {
  fn success() -> Self {
    Status {
      ok: true,
      error: String::new()
    }
  }
  
  fn error(err: ipfs_api_backend_hyper::Error) -> Self {
    dbg!(err);
    Status {
      ok: false,
      error: String::from("err")
    }
  }
}