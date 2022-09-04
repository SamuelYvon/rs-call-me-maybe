use crate::communicator::pushover::PushOverConfiguration;
use crate::communicator::libnotify::LibNotifyConfiguration;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = ".callmemaybe";
const ERR_NO_HOME_PATH: &str = "Could not find the home directory, or it does not exist";
const ERR_NO_CONFIG_DFLT_PATH : &str = "Could not find ~/.callmemaybe nor ~/.callmemaybe.toml";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub pushover: Option<PushOverConfiguration>,
    pub libnotify : Option<LibNotifyConfiguration>
}

fn resolve_file(home_dir: PathBuf) -> Option<PathBuf> {
    let wo_ext = home_dir.join(CONFIG_FILE_NAME);

    if wo_ext.exists() {
        return Some(wo_ext);
    }

    let w_ext = {
        let mut temp = home_dir.join(CONFIG_FILE_NAME);
        temp.set_extension("toml");
        temp
    };

    if w_ext.exists() {
        return Some(w_ext);
    }

    None
}

impl Config {
    pub fn resolve(
        alternative_path: Option<PathBuf>,
    ) -> Result<Config, Box<dyn std::error::Error>> {
        let pathbuf = if let Some(alt_path) = alternative_path {
            alt_path
        } else {
            let home_dir = home::home_dir();
            resolve_file(home_dir.ok_or(ERR_NO_HOME_PATH)?).ok_or(ERR_NO_CONFIG_DFLT_PATH)?
        };

        let path = pathbuf.as_path();
        let path_str = path.to_str().expect("The path must be a valid UTF-8 path");

        let file = File::open(path)
            .map_err(|_| format!("Could not find the config file at {}", path_str))?;

        let mut reader = BufReader::new(file);
        let mut contents = String::new();

        reader.read_to_string(&mut contents)?;

        let parsed: Config = toml::from_str(&contents).map_err(|err| -> String {
            format!("Failed to parse the configuration file: {err}")
        })?;

        Ok(parsed)
    }
}
