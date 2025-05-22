A bot that sends messages editable by everyone on Discord<br>
Cargo 1.87.0 + Serenity 0.12
# Setup
1. Enter your bot token to `DISCORD_BOT_TOKEN` in the `.env` file<br>
   Optionally, enter `DISCORD_GUILD_ID` if you want to use it only on a specific server
2. Run `cargo run`

# How to use
1. Use the `/create <role?>` command to create an editable message<br>
   `<role>` is the role tha can edit the message. If not specified, everyone can edit
2. Press the `Edit` button on the sent message and enter the new message content in the displayed modal 