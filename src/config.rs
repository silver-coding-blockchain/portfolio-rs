use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// overall config
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) mode: String,
    pub(crate) db: Database,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Database {
    pub(crate) host: String,
    pub(crate) port: String,
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) passwd: String,
}

impl Config {
    pub(crate) fn new() -> Config {
        Config {
            mode: "dev".to_string(),
            db: Database {
                host: "your.server.host".to_string(),
                port: "5432".to_string(),
                name: "your database name".to_string(),
                username: "your username".to_string(),
                passwd: "password".to_string(),
            },
        }
    }

    fn save(c: &Config) {
        fs::create_dir_all("config").expect("creating config dir failed");
        fs::File::create("config/config.toml").expect("creating config file failed");
        let toml_string = toml::to_string_pretty(c).expect("toml to string failed");
        fs::write("config/config.toml", toml_string).expect("writing config file failed");
    }

    pub(crate) fn read() -> Config {
        let config_file = PathBuf::from("config/config.toml");

        let f = fs::read_to_string(config_file);

        match f {
            Ok(content) => {
                let config: Config = toml::from_str(&content).expect("parsing toml error");
                config
            }
            Err(e) => {
                let c = Config::new();
                Config::save(&c);
                panic!("Please edit your config file before running {}", e);
            }
        }
    }
}