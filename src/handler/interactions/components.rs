use serenity::all::{ComponentInteraction, Context};

mod edit;

pub async fn handle_components(ctx: &Context, component: &ComponentInteraction) {
    if component.data.custom_id.starts_with("edit") {
        edit::edit(ctx, component).await;
    } else {
        println!("component not found");
    }
}