use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, Configuration, CommandResult};

#[group]
#[commands(ping)]
#[commands(dong)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("!"));

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn dong(ctx: &Context, msg: &Message) -> CommandResult{
    //println!("{}", msg.content.split(pat));
    // let s: msg.content.splitn(2," ")[1];
    // match s.collect().parse::<i32>() {
    //     Ok(n) => msg.reply(ctx, add!(n, 5)).await?,
    //     Err(e) => msg.reply(ctx, e).await?,
    // }
    msg.reply(ctx, "dong").await?;

    Ok(())
}