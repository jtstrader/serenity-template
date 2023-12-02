//! # Serenity Template
//! This template code is sourced from Serenity's GitHub repository README.
//! This code, along with additional information about Serenity, can be found
//! [here](https://github.com/serenity-rs/serenity).

mod secrets;
mod utils;

use std::process::exit;

use log;
use utils::{gcp, logging};

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, Configuration, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logging::setup_logger()?;

    let gcp_listener_handle = tokio::spawn(async {
        if let Err(e) = gcp::CloudRunListener::default().listen().await {
            log::error!("{}", e);
            exit(1);
        }
    });

    // Login with a bot token from the secrets location.
    let token = match secrets::get_discord_token() {
        Ok(token) => token,
        Err(e) => {
            log::error!("Could not obtain discord token with error: {}", e);
            exit(1);
        }
    };

    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("~")); // Set the bot's prefix to "~".

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Start listening for events by starting a single shard.
    log::info!("Discord client created. Listening for requests.");
    if let Err(why) = client.start().await {
        log::error!("An error occurred while running the client: {:?}", why);
        gcp_listener_handle.abort();
    }

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
