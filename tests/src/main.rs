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

  client.use_create_bot(String::from("coolbotusername")).await?;

  if let Err(err) = client.start("61d5e75b-bc42-42ea-84bc-38a205482d3f").await {
    eprintln!("Client failed to start. {}", err.to_string());
  }

  Ok(())
}
