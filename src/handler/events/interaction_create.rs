use serenity::all::{
    Context,
    Interaction
};

use crate::handler::interactions::{
    commands::handle_commands,
    components::handle_components
};

pub async fn interaction_create(ctx: &Context, interaction: Interaction) {
    let config = crate::config::get();
    
    match interaction {
        // Slash commands
        Interaction::Command(command) => handle_commands(&ctx, &command).await,
        Interaction::Component(component) => handle_components(&ctx, &component).await,
        _ => { println!("interaction not found"); }
    }
}
