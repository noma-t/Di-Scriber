use serenity::{async_trait};
use serenity::prelude::{Context, EventHandler};
use serenity::model::gateway::Ready;
use crate::handler::events::ready;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(&ctx, ready).await;
    }
}