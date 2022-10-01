use std::env;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::Client;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // For when the bot receives a message
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
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
