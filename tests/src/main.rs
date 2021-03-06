use dogehouse_rs::prelude::*;
use dotenv::dotenv;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn on_message(&self, msg: String) {
    println!("MESSAGE: {}\n", msg);
  }

  async fn on_any(&self, any: String) {
    println!("ANY: {}\n", any);
  }

  async fn on_pong(&self) {
    println!("Received ping");
  }

  async fn connection_closed(&self) {
    println!("Connection has closed");
    std::process::exit(1);
  }

  async fn on_ready(&self, user: &User) {
    println!("{} is ready", user.display_name);
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  dotenv().ok();

  let mut client = Client::new(
    env::var("TOKEN")?,
    env::var("REFRESH_TOKEN")?
  ).add_event_handler(Handler);

  // client.use_create_bot("i_am_wills_bot", true).await?;

  if let Err(err) = client.start(&env::var("ROOM_ID")?).await {
    eprintln!("Client failed to start. {}", err.to_string());
  }

  Ok(())
}
