use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
}

impl Config {
    /// Reads the config file. Creates a default one if it doesn't exist
    pub async fn new() -> Self {
        let exists = fs::try_exists("talky.toml")
            .await
            .expect("Unable to check if config file exists");

        if exists {
            toml::from_str(&fs::read_to_string("talky.toml").await.unwrap()).expect("Invalid config file! Make sure it is valid toml syntax and required fields are filled.")
        } else {
            const DEFAULT_CONFIG: &str = include_str!("default.toml");
            fs::write("talky.toml", DEFAULT_CONFIG)
                .await
                .expect("Unable to create config file");

            panic!("Config file created, please edit it and restart the program");
        }
    }
}
