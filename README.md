# dogehouse-rs

## WARNING: Still work in progress do not use yet

#### Example
```rust
use dogehouse_rs::prelude::*;
use dogehouse_rs::Message;
use dotenv::dotenv;
use std::env;

struct Handle;

impl Handler for Handle { 
  
  fn on_ready(&self, user: String) {
    println!("{}", user);
  }
  
  fn on_message(&self, msg: &Message) {
    println!("{:?}", msg); 
  }
}

fn main() {
  dotenv().ok();
  let token = env::var("token")
    .expect("could not find token");

  let refresh_token = env::var("refresh_token")
    .expect("could not find token");

  let mut client = Client::new(token, refresh_token)
    .add_handler(Handle);

                                 // Room id
  if let Err(err) = client.start("3daf5a80-5b0a-4dde-9527-9db1f7f13755") {
    println!("{}", err.to_string());
  }
}

```

#### Testing
```shell
cargo test -- --nocapture
```