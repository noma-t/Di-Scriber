use serenity::all::{
    Interaction,
    async_trait,
    Context,
    EventHandler,
    Ready,
};
use crate::handler::events::{
    ready, interaction_create
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(&ctx, ready).await;
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::interaction_create(&ctx, interaction).await;
    }
}