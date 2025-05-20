use serenity::all::CommandOptionType;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::prelude::{
    Context,
};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;

pub async fn ready(ctx: &Context, ready: Ready) {
    let config = crate::config::get();
    println!("{}",
        config.event.ready.output
            .replace("{name}", &ready.user.name)
            .replace("{id}", &ready.user.id.to_string())
    );
    
    let guild_id = GuildId::new(crate::config::get().discord.guild_id);
    
    let commands = guild_id
        .set_commands(
            &ctx.http,
            vec![CreateCommand::new(&config.command.create.name)
                .description(&config.command.create.description)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Role,
                        &config.command.create.options.role.name,
                        &config.command.create.options.role.description,
                    )
                    .required(false),
                )],
        )
        .await;
    match commands {
        Ok(_) => println!("Commands registered successfully"),
        Err(err) => eprintln!("Error registering commands: {:?}", err),
    }
}