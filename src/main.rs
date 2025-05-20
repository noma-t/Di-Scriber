mod handler;
mod config;

use serenity::{Client};
use serenity::prelude::{GatewayIntents};
use handler::handler::Handler;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init()?;
    
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    let mut client = Client::builder(&config::get().discord.bot_token, intents)
        .event_handler(Handler)
        .await?;
    
    client.start().await?;
    
    Ok(())
}
