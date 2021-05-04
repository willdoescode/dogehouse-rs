pub mod prelude;
pub(crate) static API_URL: &str = "wss://api.dogehouse.tv/socket";
pub(crate) static AUTH_GOOD: &str = "auth-good";
use std::sync::{Arc, RwLock};
use url::Url;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use async_trait::async_trait;
use futures::{StreamExt, SinkExt};
use serde_json::json;

#[async_trait]
pub trait EventHandler {
	async fn on_message(&self, _msg: String);
	async fn connection_closed(&self);
}

#[derive(Clone)]
pub struct Client<'a, T>
	where T: EventHandler + Sync
{
	token: String,
	refresh_token: String,
	room_id: Option<&'a str>,
	event_handler: Option<T>
}

impl<'a, T> Client<'a, T> where T: EventHandler + Sync {
	pub fn new(token: String, refresh_token: String) -> Self {
		Self {
			token,
			refresh_token,
			room_id: None,
			event_handler: None
		}
	}

	/// Let user assign an event handler that implements EventHandler trait
	pub fn add_event_handler(&mut self, handler: T) {
		self.event_handler = Some(handler);
	}

	pub async fn start(&mut self, room_id: &'a str) -> anyhow::Result<()> {
		self.room_id = Some(room_id);
		let url = Url::parse(API_URL)?;
		println!("Connecting to {}", url);

		let (ws_stream, _) = connect_async(url)
			.await
			.expect("Failed to connect");

		println!("Successfully connected.\n");

		let (mut write, mut read) = ws_stream.split();

		write.send(Message::Text(json!({
			"op": "auth",
			"d": {
				"accessToken": self.token,
				"refreshToken": self.refresh_token,
				"reconnectToVoice": false,
				"currentRoomId": self.room_id.unwrap(),
				"muted": true,
				"platform": "dogehouse-rs"
			}
		}).to_string())).await?;

		tokio::spawn(async move {
			loop {
				write.send("ping".into()).await.unwrap();
				std::thread::sleep(std::time::Duration::new(8, 0));
			}
		});

		while let Some(msg) = read.next().await {
			let msg = msg?;
			if msg.is_close() {
				self.event_handler.as_ref().unwrap().connection_closed().await;
				continue;
			}
			self.event_handler.as_ref().unwrap().on_message(msg.to_string()).await;
		}

		Ok(())
	}
}
