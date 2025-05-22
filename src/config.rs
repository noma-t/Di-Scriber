use config::{Config, ConfigError};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;
use std::sync::RwLock;

#[derive(Debug, Clone, Deserialize)]
pub struct Configs {
    pub message: MessageConfig,
    pub event: EventConfig,
    pub command: CommandConfig,
    pub modal: ModalConfig,
    #[serde(skip)]
    pub discord: DiscordConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageConfig {
    pub content: String,
    pub button: MessageButtonConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageButtonConfig {
    pub label: String,
    pub missing_role: String
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DiscordConfig {
    pub bot_token: String,
    pub guild_id: u64
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventConfig {
    pub ready: ReadyEventConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReadyEventConfig {
    pub output: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandConfig {
    pub create: CreateCommandConfig
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommandConfig {
    pub name: String,
    pub description: String,
    pub success_message: String,
    pub options: CreateCommandOptionsConfig
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommandOptionsConfig {
    pub role: CreateCommandRoleOptionConfig
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommandRoleOptionConfig {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModalConfig {
    pub edit: EditModalConfig
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditModalConfig {
    pub title: String,
    pub input: EditModalInputConfig
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditModalInputConfig {
    pub label: String,
    pub placeholder: String,
}

pub static CONFIG: Lazy<RwLock<Option<Configs>>> = Lazy::new(|| RwLock::new(None));

pub fn init() -> Result<(), ConfigError> {
    dotenv().ok();
    
    let config_builder = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .add_source(config::Environment::with_prefix("").separator("_"))
        .build()?;
    
    let mut app_config: Configs = config_builder.try_deserialize()?;
    
    app_config.discord = DiscordConfig {
        bot_token: env::var("DISCORD_BOT_TOKEN")
            .expect("DISCORD_BOT_TOKEN must be set in .env file"),
        guild_id: env::var("DISCORD_GUILD_ID")
            .ok()
            .and_then(|id| {
                if id.trim().is_empty() {
                    None
                } else {
                    id.parse::<u64>().ok()
                }
            })
            .unwrap_or(0)
    };
    
    let mut config_writer = CONFIG.write().unwrap();
    *config_writer = Some(app_config.clone());
    Ok(())
}

pub fn get() -> Configs {
    let config_reader = CONFIG.read().unwrap();
    match &*config_reader {
        Some(cfg) => cfg.clone(),
        None => panic!("Config not initialized. Call init() first."),
    }
}