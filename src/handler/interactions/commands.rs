use serenity::all::{CommandInteraction, Context};

mod create;

pub async fn handle_commands(ctx: &Context, command: &CommandInteraction) {
    match command.data.name.as_str() {
        "create" => create::create(&ctx, &command).await,
        _ => Ok({ println!("commands not found"); })
    }.expect("TODO: panic message");
}