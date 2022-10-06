use std::collections::HashMap;
use std::env;

use reqwest::StatusCode;
use serenity::{async_trait, json};
use serenity::http::{Http, CacheHttp};
use serenity::json::json;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::model::id::{ChannelId, MessageId};
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

        println!("I got a message. Sending it to the server.");

        let request = reqwest::Client::new();

        if msg.author.name == "Bourbon" {
            match msg.content.split(" ").nth(0).unwrap() {
                "!ping" => { 
                    msg.channel_id.say(ctx.http, "Pong!").await;
                },

                "!post" => {
                    // TODO: Find a way to clean this up.
                    let message_ids: Vec<&str> = msg.content.split(" ")
                        .collect();

                    let message_ids: Vec<MessageId> = message_ids[1..]
                        .iter()
                        .map(|id| id.parse::<u64>().unwrap())
                        .map(|id| MessageId::from(id))
                        .collect();

                    let channel_id = msg.channel_id;

                    // TODO: Get messages, make JSON and post them to the server
                    let messages: Vec<Message> = get_messages_by_id(&ctx, message_ids, channel_id).await.unwrap();
                    post_messages_to_server(ctx, messages).await;
                },

                _ => {
                    println!("Received a non-command");
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn post_messages_to_server(ctx: Context, messages: Vec<Message>) {
    let request = reqwest::Client::new();

    // TODO: Remove hardcoded IDs
    let post = json!({
        "id": 0 as usize,
        "platform": "Discord",
        "messages": messages,
    });

    let res = request.post("http://0.0.0.0:8080/message_from_discord")
        .json(&post)
        .send().await.unwrap();

    dbg!(post);

    match res.status() {
        StatusCode::OK => {
            messages.get(0).unwrap().
                channel_id.say(ctx.http, "Posted your messages to Chronicles!")
                .await.unwrap();
        },
        // TODO: Write more granular cases for failed requests.
        _ => {
            messages.get(0).unwrap().channel_id.
                say(ctx.http, format!("Post to Chronicles failed because: {}", res.text().await.unwrap()))
                .await.unwrap();
        },
    }
}

async fn get_messages_by_id(ctx: &Context, message_ids: Vec<MessageId>, channel_id: ChannelId) -> Option<Vec<Message>>{
    let mut messages: Vec<Message> = Vec::new();

    for id in message_ids {
        messages.push(ctx.http.get_message( channel_id.0, id.0).await.unwrap());
    }

    return Some(messages);
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
