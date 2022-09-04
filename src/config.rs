use crate::communicator::pushover::PushOverConfiguration;
use home;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use toml;

const CONFIG_FILE_NAME: &str = ".callmemaybe";
const ERR_NO_HOME_PATH: &str = "Could not find the home directory, or it does not exist";
const ERR_NO_CONF_FILE: &str = "Could not find the config file (~/.callmemaybe).";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub pushover: Option<PushOverConfiguration>,
}

impl Config {
    pub fn resolve() -> Result<Config, Box<dyn std::error::Error>> {
        let home_dir = home::home_dir();
        let path = home_dir.ok_or(ERR_NO_HOME_PATH)?.join(CONFIG_FILE_NAME);

        let file = File::open(path).map_err(|_| -> &str { ERR_NO_CONF_FILE })?;

        let mut reader = BufReader::new(file);
        let mut contents = String::new();

        reader.read_to_string(&mut contents)?;

        let parsed: Config = toml::from_str(&contents).map_err(|err| -> String {
            format!("Failed to parse the configuration file: {err}")
        })?;

        return Ok(parsed);
    }
}
