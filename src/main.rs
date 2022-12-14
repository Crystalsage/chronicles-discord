use std::env;

use serde::Serialize;

use reqwest::StatusCode;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::model::id::{ChannelId, MessageId};
use serenity::prelude::*;
use serenity::Client;

struct Handler;

#[derive(Serialize, Debug)]
struct Messages {
    messages: Vec<String>
}

static HOST: &'static str = "http://0.0.0.0:8080";

// We'll consider the 
// - Timestamp ==> [Date, Time]
// - Author
// - Message

#[async_trait]
impl EventHandler for Handler {

    // For when the bot receives a message
    async fn message(&self, ctx: Context, msg: Message) {

        println!("I got a message. Sending it to the server.");

        if msg.author.name == "Bourbon" {
            match msg.content.split(" ").nth(0).unwrap() {
                "!ping" => { 
                    msg.channel_id.say(ctx.http, "Pong!").await.unwrap();
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

    let channel_id: ChannelId = messages.get(0).unwrap().channel_id;

    let server_messages: Vec<String> = messages.iter()
        .map(|msg| msg.content.to_owned())
        .collect();

    // TODO: Remove hardcoded IDs
    let messages = Messages {
        messages: server_messages,
    };

    let res = request.post(HOST.to_owned() + "/create_post")
        .body(serde_json::to_string(&messages).unwrap())
        .header("Content-Type", "application/json")
        .send().await.unwrap();
    
    // dbg!(res.bytes);

    match res.status() {
        StatusCode::OK => {
                channel_id.say(ctx.http, "Posted your messages to Chronicles!")
                .await.unwrap();
        },
        // TODO: Write more granular cases for failed requests.
        _ => {
            channel_id.say(ctx.http, format!("Post to Chronicles failed because: {}", res.text().await.unwrap()))
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
