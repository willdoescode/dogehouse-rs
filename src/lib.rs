use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use uuid::Uuid;

use config::{API_URL, TIME_OUT};
use user::{TopUserWrapper, User};

mod config;
pub mod prelude;
mod user;

/// In order to use EvenHandler you must first define a struct and then
/// implement EvenHandler for that struct.
///
/// Then you must pass that struct as an argument to add_event_handler
/// which is defined on Client.
/// ```
/// #[async_trait]
/// impl EventHandler for Handler {
///   async fn on_message(&self, msg: String) {
///     println!("{}", msg);
///   }
///
///   async fn on_pong(&self) {
///     println!("Received ping")
///   }
///
///   async fn connection_closed(&self) {
///     println!("Connection has closed");
///     std::process::exit(1);
///   }
///
///   async fn on_ready(&self, user: &User) {
///     println!("{} is ready", user.display_name);
///   }
/// }
/// ```
#[async_trait]
pub trait EventHandler {
    async fn on_message(&self, _msg: String);
    async fn on_any(&self, _any: String);
    async fn on_pong(&self);
    async fn connection_closed(&self);
    async fn on_ready(&self, _user: &User);
}

#[derive(Clone)]
pub struct Client<'a, T>
where
    T: EventHandler + Sync,
{
    token: String,
    refresh_token: String,
    bot_token: Option<String>,
    bot_refresh_token: Option<String>,
    room_id: Option<&'a str>,
    event_handler: Option<T>,
}

impl<'a, T> Client<'a, T>
where
    T: EventHandler + Sync,
{
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

    /// In order to create a bot make sure to first authenticate with your normal tokens and create a client
    /// ```
    /// let mut client = Client::new(
    ///   token,
    ///   refresh_token
    /// ).add_event_handler(Handler);
    /// ```
    /// Then call use_create_bot on client with a username and a boolean value to tell dogehouse-rs
    /// if you want your bot tokens to be shown.
    /// use_create_bot is an async function so make sure to use await or a blocking call.
    /// ```
    /// client.use_create_bot("bot_name", true").await?;
    /// ```
    pub async fn use_create_bot(
        &mut self,
        username: &'a str,
        show_bot_tokens: bool,
    ) -> anyhow::Result<()> {
        let url = Url::parse(API_URL)?;

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (mut write, mut read) = ws_stream.split();

        write
            .send(Message::Text(
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
                )
                .to_string(),
            ))
            .await?;

        #[derive(Deserialize)]
        struct CreateBotResponse {
            op: String,
            p: CreateBotResponseP,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct CreateBotResponseP {
            api_key: serde_json::Value,
            is_username_taken: serde_json::Value,
            error: serde_json::Value,
        }

        write
            .send(Message::Text(
                json!(
                    {
                        "op": config::user::CREATE_BOT,
                        "p": {
                            "username": username
                        },
                        "ref": "[uuid]",
                        "v": "0.2.0",
                    }
                )
                .to_string(),
            ))
            .await?;

        // Skip messages from first two requests
        read.next().await;
        read.next().await;

        let n = read.next().await.unwrap()?.to_string();
        let bot_response =
            serde_json::from_str::<CreateBotResponse>(&n).expect("Error invalid bot name");
        if bot_response.p.is_username_taken.is_boolean() {
            return Err(anyhow::Error::msg("Bot name is taken."));
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct BotAccount {
            access_token: String,
            refresh_token: String,
        }

        let bot_account = reqwest::Client::new()
            .post("https://api.dogehouse.tv/bot/auth")
            .header("content-type", "application/json")
            .body(
                json!(
                    {
                        "apiKey": bot_response.p.api_key.as_str()
                    }
                )
                .to_string(),
            )
            .send()
            .await?
            .json::<BotAccount>()
            .await?;

        if show_bot_tokens {
            println!("Bot tokens: {:?}", &bot_account);
        }

        self.bot_token = Some(bot_account.access_token);
        self.bot_refresh_token = Some(bot_account.refresh_token);
        Ok(())
    }

    /// Let user assign an event handler that implements EventHandler trait
    pub fn add_event_handler(mut self, handler: T) -> Self {
        self.event_handler = Some(handler);
        self
    }

    pub async fn start(&mut self, room_id: &'a str) -> anyhow::Result<()> {
        self.room_id = Some(room_id);

        let token = match self.bot_token.is_some() {
            true => &self.bot_token.as_ref().unwrap(),
            false => &self.token,
        };

        let refresh_token = match self.bot_refresh_token.is_some() {
            true => &self.bot_refresh_token.as_ref().unwrap(),
            false => &self.refresh_token,
        };

        let url = Url::parse(API_URL)?;

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (mut write, mut read) = ws_stream.split();

        write
            .send(Message::Text(
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
                )
                .to_string(),
            ))
            .await?;

        let n = read.next().await.unwrap()?.to_string();
        println!("{}", &n);

        let account: TopUserWrapper = serde_json::from_str(&n)?;
        self.event_handler
            .as_ref()
            .unwrap()
            .on_ready(&account.d.user)
            .await;

        let room_join_ref = Uuid::new_v4();

        write
            .send(Message::Text(
                json!({
                    "op": "room:join",
                    "d": {
                        "roomId": self.room_id.unwrap()
                    },
                    "ref": "[uuid]",
                    "v": "0.2.0",
                })
                .to_string(),
            ))
            .await?;

        tokio::spawn(async move {
            loop {
                write.send("ping".into()).await.unwrap();
                std::thread::sleep(std::time::Duration::new(TIME_OUT, 0));
            }
        });

        #[derive(Debug, Deserialize)]
        struct Response {
            op: String,
            d: serde_json::Value,
        }

        while let Some(msg) = read.next().await {
            let msg = msg?;
            if msg.is_close() {
                self.event_handler
                    .as_ref()
                    .unwrap()
                    .connection_closed()
                    .await;
                continue;
            } else if msg.is_binary() || msg.is_text() {
                let beginning_parsed_json =
                    serde_json::from_str(&msg.to_string()).unwrap_or(Response {
                        op: "no".to_string(),
                        d: serde_json::Value::String("no".into()),
                    });

                println!("BEGINNING PARSED JSON {:?}\n", &beginning_parsed_json);

                if beginning_parsed_json.op == "new_chat_msg".to_string() {
                    self.event_handler
                        .as_ref()
                        .unwrap()
                        .on_message(
                            beginning_parsed_json
                                .d
                                .as_object()
                                .unwrap()
                                .iter()
                                .map(|x| format!("{}", x.0))
                                .collect::<String>(),
                        )
                        .await;
                }

                if msg.to_string() == "pong" {
                    self.event_handler.as_ref().unwrap().on_pong().await;
                    continue;
                }

                self.event_handler
                    .as_ref()
                    .unwrap()
                    .on_any(msg.to_string())
                    .await;
                continue;
            }
        }

        Ok(())
    }
}
