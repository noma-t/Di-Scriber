use serenity::all::{
    CommandOptionType,
    CreateCommand,
    CreateCommandOption,
    Ready,
    GuildId,
    Context
};
use crate::config;

pub async fn ready(ctx: &Context, ready: Ready) {
    let config = config::get();
    
    // Print the bot's name and ID
    println!("{}",
        config.event.ready.output
            .replace("{name}", &ready.user.name)
            .replace("{id}", &ready.user.id.to_string())
    );
    
    let command = CreateCommand::new(&config.command.create.name)
        .description(&config.command.create.description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Role,
                &config.command.create.options.role.name,
                &config.command.create.options.role.description,
            )
            .required(false),
        );
    
    // Register slash commands
    let registered_commands = if config.discord.guild_id != 0 {
        let guild_id = GuildId::new(config.discord.guild_id);
        guild_id.create_command(&ctx.http, command).await
    } else {
        ctx.http.create_global_command(&command).await
    };
    
    // Handle the result of the command registration
    match registered_commands {
        Ok(_) => println!("Commands registered successfully"),
        Err(err) => eprintln!("Error registering commands: {:?}", err),
    }
}