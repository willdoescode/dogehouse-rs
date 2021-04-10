use dogehouse_rs::prelude::*;
use dogehouse_rs::message::Message;

struct Handle;

impl Handler for Handle {
  fn on_ready(&self, user: String) { println!("{}", user); }

  fn on_message(&self, msg: &Message) { println!("{:?}", msg); }
}

fn main() {
  let mut client = Client::new("hello", "world")
    .add_handler(Handle);

  if let Err(err) = client.start() {
    println!("{}", err.to_string());
  }
  println!("Hello, world!");
}
