use std::sync::OnceLock;

use figment::providers::{Env, Format, Toml};
use figment::Figment;
use serde::Deserialize;

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

pub fn init() {
    let raw_config = Figment::new()
        .merge(Toml::file(
            Env::var("APP_CONFIG").as_deref().unwrap_or("config.toml"),
        ))
        .merge(Env::prefixed("APP_").global());

    let config = match raw_config.extract::<ServerConfig>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("It looks like your config is invalid. The following error occurred: {e}");
            std::process::exit(1);
        }
    };
    
    crate::config::CONFIG
        .set(config)
        .expect("config should be set");
}

pub fn get() -> &'static ServerConfig {
    CONFIG.get().expect("config should be set")
}

#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,

    pub jwt: JwtConfig,
}

fn default_listen_addr() -> String {
    "[::]:5701".into()
}