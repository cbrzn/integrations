use ipfs_api_backend_actix::{IpfsClient, IpfsApi};
use std::fs::File;
use std::path::Path;

struct Status {
  ok: bool,
  error: String
}

#[actix_rt::main]
pub async fn main() {
  let client = IpfsClient::default();
  // Call into_bytes() which returns a Vec<u8>, and iterate accordingly
  // I only called clone() because this for loop takes ownership
  let name = "./test-f".to_string();
  let mut name_in_binary: Vec<u8> = vec!();
  for character in name.clone().into_bytes() {
      name_in_binary.push(character);
  }
  let path = std::str::from_utf8(&name_in_binary).unwrap();
  match client.add_path(path).await {
    
    Ok(file) => {
      dbg!(file);
      ()
    },
    Err(e) => {
      dbg!(e);
      todo!()
    }
  }
}


impl Status {
  fn success() -> Self {
    Status {
      ok: true,
      error: String::new()
    }
  }
  
  fn error(err: ipfs_api_backend_actix::Error) -> Self {
    dbg!(err);
    Status {
      ok: false,
      error: String::from("err")
    }
  }
}