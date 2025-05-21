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
    
    // Register slash commands
    let guild_id = GuildId::new(config.discord.guild_id);
    
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
    
    // Handle the result of the command registration
    match commands {
        Ok(_) => println!("Commands registered successfully"),
        Err(err) => eprintln!("Error registering commands: {:?}", err),
    }
}