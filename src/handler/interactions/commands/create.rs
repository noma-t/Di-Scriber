use serenity::all::{ActionRowComponent, ButtonStyle, CommandInteraction, Context, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateMessage};

pub async fn create(ctx: &Context, command: &CommandInteraction) -> Result<(), serenity::Error> {
    let config = crate::config::get();

    
    let mut custom_id = String::from("edit");
    
    if let Some(role_option) = command.data.options.iter()
        .find(|opt| opt.name == config.command.create.options.role.name) {
        if let Some(role_id_obj) = role_option.value.as_role_id() {
            custom_id = format!("edit:{}", role_id_obj.get());
        }
    }
    
    command.channel_id.send_message(&ctx.http,
        CreateMessage::new()
            .content(config.message.content)
            .components(vec![
                CreateActionRow::Buttons(vec![
                    CreateButton::new(custom_id)
                        .label(config.message.button.label)
                        .style(ButtonStyle::Primary)
                ])
            ])
    )
    .await?;
    
    command.create_response(&ctx.http,
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content(config.command.create.success_message)
                .ephemeral(true)
        )
    ).await?;
    
    Ok(())
}