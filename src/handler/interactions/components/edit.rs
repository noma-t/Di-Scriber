use serenity::all::{ComponentInteraction, Context, CreateActionRow, CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, InputTextStyle};
use crate::config;

pub async fn edit(ctx: &Context, component: &ComponentInteraction){
    let config = config::get();
    
    if let Some(role_id) = component.data.custom_id.split(':').collect::<Vec<&str>>().get(1) {
        if component.user.has_role(&ctx.http, component.guild_id.unwrap(), role_id.parse::<u64>().unwrap()).await.unwrap() == false {
            component.create_response(&ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(config.message.button.missing_role.replace("{role}", format!("<@&{}>", role_id).as_str()))
                        .ephemeral(true)
                )
            ).await.expect("Could not create interaction response");
            return;
        };
    }
    
    
    component.create_response(&ctx.http,
        CreateInteractionResponse::Modal(
            CreateModal::new(format!("edit:{}:{}", component.channel_id, component.message.id), config.modal.edit.title)
                .components(vec![
                    CreateActionRow::InputText(
                        CreateInputText::new(
                            InputTextStyle::Paragraph,
                            &config.modal.edit.input.label,
                            &config.modal.edit.input.placeholder
                        )
                        .value(&component.message.content)
                    )
                ]
            )
        )
    )
    .await
    .expect("Failed to create modal response");
}