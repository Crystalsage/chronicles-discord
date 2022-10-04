use std::collections::HashMap;
use std::env;

use serenity::async_trait;
use serenity::json::json;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::Client;


struct Handler;


// We'll consider the 
// - Timestamp ==> [Date, Time]
// - Author
// - Message

#[async_trait]
impl EventHandler for Handler {
    // For when the bot receives a message
    async fn message(&self, ctx: Context, msg: Message) {
        let latest_message = json!({
            "timestamp": msg.timestamp.unix_timestamp() as usize,
            "content": msg.content,
            "author": msg.author.name
        });

        println!("I got a message. Sending it to the server.");

        let request = reqwest::Client::new();

        if msg.author.name == "Bourbon" {
            let res = request.post("http://0.0.0.0:8080/message_from_discord")
                .json(&latest_message)
                .send().await.unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
    .expect("Discord token not found in environment");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
    .event_handler(Handler)
    .await
    .expect("Client creation failed!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
