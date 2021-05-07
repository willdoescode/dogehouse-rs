use dogehouse_rs::prelude::*;
use dotenv::dotenv;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn on_message(&self, msg: String) {
    println!("{}", msg);
  }

  async fn on_pong(&self) {
    println!("Received ping")
  }

  async fn connection_closed(&self) {
    println!("Connection has closed");
    std::process::exit(1);
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  dotenv().ok();

  let mut client = Client::new(
    env::var("TOKEN").unwrap(),
    env::var("REFRESH_TOKEN").unwrap()
  ).add_event_handler(Handler);

  // client.use_create_bot("i_am_wills_bot", true).await?;

  if let Err(err) = client.start("975be804-afbc-40c3-a2ba-5494bb2af788").await {
    eprintln!("Client failed to start. {}", err.to_string());
  }

  Ok(())
}
