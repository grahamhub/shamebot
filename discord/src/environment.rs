use log::warn;
use std::env;

pub struct Env {
    pub discord_token: String,
    pub shamebot_url: String,
}

impl Env {
    pub fn new() -> Self {
        let discord_token = env::var("SHAMEBOT_DISCORD_TOKEN")
            .map_err(|_| warn!("environment variable SHAMEBOT_DISCORD_TOKEN not set"))
            .unwrap_or_default();
        let shamebot_url = env::var("SHAMEBOT_URL")
            .map_err(|_| warn!("environment variable SHAMEBOT_URL not set"))
            .unwrap_or_default();

        Env {
            discord_token,
            shamebot_url,
        }
    }
}
