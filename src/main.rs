use std::collections::HashMap;
use std::env;

use serenity::async_trait;
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
        let mut latest_message = HashMap::new();

        latest_message.insert("timestamp", msg.timestamp.unix_timestamp().to_string());
        latest_message.insert("content", msg.content);
        latest_message.insert("author", "Bourbon".to_string());

        println!("I got a message. Replying to it");

        let request = reqwest::Client::new();

        if msg.author.name == "Bourbon" {
            request.post("http://127.0.0.1:8080")
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
