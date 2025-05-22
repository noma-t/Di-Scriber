use serenity::all::{ChannelId, Context, EditMessage, MessageId, ModalInteraction, ActionRowComponent, CreateInteractionResponse};

pub async fn edit(ctx: &Context, modal: &ModalInteraction) {
    let custom_id_parts: Vec<&str> = modal.data.custom_id.split(':').collect();
    let channel_id = ChannelId::new(custom_id_parts[1].parse::<u64>().unwrap());
    let message_id = MessageId::new(custom_id_parts[2].parse::<u64>().unwrap());
    
    let new_content = modal
        .data
        .components
        .get(0)
        .and_then(|c| c.components.get(0))
        .and_then(|c| {
            if let ActionRowComponent::InputText(input_text) = c {
                input_text.value.as_ref()
            } else {
                None
            }
        })
        .unwrap();
    
    channel_id.edit_message(&ctx.http, message_id,
        EditMessage::new()
            .content(new_content)
    ).await.expect("Failed to edit message");
    
    modal.create_response(&ctx.http,
        CreateInteractionResponse::Acknowledge
    ).await.expect("Failed to acknowledge modal");
    
}