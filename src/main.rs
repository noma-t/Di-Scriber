mod handler;
mod config;

use serenity::{Client};
use serenity::prelude::{GatewayIntents};
use handler::handler::Handler;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init()?;

    let intents = GatewayIntents::default();

    let mut client = Client::builder(&config::get().discord.bot_token, intents)
        .event_handler(Handler)
        .await?;

    client.start().await?;

    Ok(())
}
