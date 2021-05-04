# dogehouse-rs

## WARNING: Still work in progress do not use yet

#### Example
##### In Cargo.toml
```toml
dogehouse-rs = "*"
```

##### In src/main.rs

```rust
use dogehouse_rs::prelude::*;
use dotenv::dotenv;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn on_message(&self, msg: String) {
		println!("{}", msg);
	}

	async fn connection_closed(&self) {
		println!("Connection has closed")
	}
}

#[tokio::main]
async fn main() {
	dotenv().ok();

	let mut client = Client::new(
		env::var("TOKEN").unwrap(),
		env::var("REFRESH_TOKEN").unwrap()
	);

	client.add_event_handler(Handler);

	if let Err(err) = client.start("9d48a1ad-1205-4626-9de9-be61c347c798").await {
		eprintln!("Client failed to start. {}", err.to_string());
	}
}
```

#### Testing
```shell
cargo test -- --nocapture
```