/// Crate imports
pub mod prelude;
mod opcodes;

/// External imports
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use async_trait::async_trait;
use futures::{StreamExt, SinkExt};
use serde_json::json;
use serde::Deserialize;
use url::Url;

/// Constants
const API_URL: &'static str = "wss://api.dogehouse.tv/socket";

/// Implement EventHandler for a struct that you have defined
/// ```
/// struct Handler;
///
/// #[async_trait]
/// impl EventHandler for Handler {
/// 	async fn on_message(&self, msg: String) {
/// 		println!("{}", msg);
/// 	}
///
/// 	async fn connection_closed(&self) {
/// 		println!("Connection closed");
///     }
///
/// 	async fn on_pong(&self) {
/// 		println!("Received pong")
/// 	}
/// }
/// ```
#[async_trait]
pub trait EventHandler {
	async fn on_message(&self, _msg: String);
	async fn on_pong(&self);
	async fn connection_closed(&self);
}

#[derive(Clone)]
pub struct Client<'a, T>
	where T: EventHandler + Sync
{
	token: String,
	refresh_token: String,
	bot_token: Option<String>,
	bot_refresh_token: Option<String>,
	room_id: Option<&'a str>,
	event_handler: Option<T>,
}

impl<'a, T> Client<'a, T> where T: EventHandler + Sync {
	pub fn new(token: String, refresh_token: String) -> Self {
		Self {
			token,
			refresh_token,
			bot_token: None,
			bot_refresh_token: None,
			room_id: None,
			event_handler: None,
		}
	}

	pub async fn use_create_bot(&mut self, username: String) -> anyhow::Result<()> {
		let url = Url::parse(API_URL)?;
		println!("pub async Connecting to {}", url);

		let (ws_stream, _) = connect_async(url)
			.await
			.expect("Failed to connect");

		println!("Successfully connected.\n");

		let (mut write, mut read) = ws_stream.split();

		write.send(Message::Text(
			json!(
			{
				"op": "auth",
				"d": {
					"accessToken": self.token,
					"refreshToken": self.refresh_token,
					"reconnectToVoice": false,
					"currentRoomId": "",
					"muted": true,
					"platform": "dogehouse-rs"
				}
			}
		).to_string()
		)).await?;

		#[derive(Deserialize)]
		struct CreateBotResponse {
			op: String,
			p: CreateBotResponseP,
		}

		#[derive(Deserialize)]
		struct CreateBotResponseP {
			apiKey: serde_json::Value,
			isUsernameTaken: serde_json::Value,
			error: serde_json::Value
		}

		write.send(Message::Text(
			json!(
			{
				"op": opcodes::user::CREATE_BOT,
				"p": {
					"username": username
				},
				"ref": "[uuid]",
				"v": "0.2.0",
			}
		).to_string()
		)).await?;

		/// Skip messages
		read.next().await;
		read.next().await;

		let n = read.next().await.unwrap()?.to_string();
		let bot_response = serde_json::from_str::<CreateBotResponse>(&n)?;
		if bot_response.p.isUsernameTaken.is_boolean() {
			return Err(anyhow::Error::msg("Bot name is taken."));
		}

		#[derive(Deserialize, Debug)]
		struct BotAccount {
			accessToken: String,
			refreshToken: String
		}

		let bot_account = reqwest::Client::new()
			.post("https://api.dogehouse.tv/bot/auth")
			.header("content-type", "application/json")
			.body(json!({
			"apiKey": bot_response.p.apiKey.as_str()
		}).to_string())
			.send()
			.await?
			.json::<BotAccount>()
			.await?;

		self.bot_token = Some(bot_account.accessToken);
		self.bot_refresh_token = Some(bot_account.refreshToken);
		Ok(())
	}

	/// Let user assign an event handler that implements EventHandler trait
	pub fn add_event_handler(mut self, handler: T) -> Self {
		self.event_handler = Some(handler);
		self
	}

	pub async fn start(&mut self, room_id: &'a str) -> anyhow::Result<()> {
		println!("{}", opcodes::test::OPERATOR);
		self.room_id = Some(room_id);
		let mut token = "";
		let mut refresh_token = "";
		if self.bot_token.is_some() {
			token = &self.bot_token.as_ref().unwrap();
			refresh_token = &self.bot_refresh_token.as_ref().unwrap();
		} else {
			token = &self.token;
			refresh_token = &self.refresh_token;
		}

		let url = Url::parse(API_URL)?;
		println!("Connecting to {}", url);

		let (ws_stream, _) = connect_async(url)
			.await
			.expect("Failed to connect");

		println!("Successfully connected.\n");

		let (mut write, mut read) = ws_stream.split();

		write.send(Message::Text(
			json!(
				{
					"op": "auth",
					"d": {
						"accessToken": token,
						"refreshToken": refresh_token,
						"reconnectToVoice": false,
						"currentRoomId": self.room_id.unwrap(),
						"muted": true,
						"platform": "dogehouse-rs"
					}
				}
			).to_string()
		)).await?;

		write.send(Message::Text(
			json!({
				"op": "room:join",
				"d": {
					"roomId": self.room_id.unwrap()
				}
			}).to_string()
		)).await?;

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

			else if msg.is_binary() || msg.is_text() {
				if msg.to_string() == "pong" {
					self.event_handler.as_ref().unwrap().on_pong().await;
					continue;
				}
				self.event_handler.as_ref().unwrap().on_message(msg.to_string()).await;
				continue;
			}
		}

		Ok(())
	}
}
