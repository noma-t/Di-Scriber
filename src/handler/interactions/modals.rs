use serenity::all::{Context, ModalInteraction};

mod edit;


pub async fn handle_modals(ctx: &Context, modal: &ModalInteraction) {
    if modal.data.custom_id.starts_with("edit:") {
        edit::edit(ctx, modal).await;
    } else {
        println!("Modal not found");
    }
}