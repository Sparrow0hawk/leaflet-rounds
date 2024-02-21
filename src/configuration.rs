use config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub host: String,
    pub port: i32,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to get current directory");
    let settings = config::Config::builder()
        .add_source(config::File::from(base_path.join("config.toml")))
        .build()?;

    settings.try_deserialize::<Settings>()
}
