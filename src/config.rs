use std::{
    fs,
    io::{Read, Write},
    sync::Arc,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIG: Arc<Config> = Arc::new(Config::new());
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,
}

impl Config {
    pub fn new() -> Config {
        let mut config_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("./config.toml")
            .expect("Error with config file! Cannot create or open the file!");
        let mut contents = String::new();
        config_file.read_to_string(&mut contents).unwrap();

        if contents.is_empty() {
            let default_config = Config {
                token: String::from("insert_token_here"),
            };

            config_file
                .write_all(toml::to_string_pretty(&default_config).unwrap().as_bytes())
                .expect("Unable to write default config to file!");
            panic!("Please fill out the config file and restart the bot");
        }
        toml::from_str(&contents).unwrap()
    }
}
