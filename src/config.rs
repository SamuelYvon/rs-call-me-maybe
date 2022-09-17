#[cfg(feature = "libinotify")]
use crate::communicator::libnotify::LibNotifyConfiguration;
use crate::communicator::pushover::PushOverConfiguration;
use crate::communicator::httppost::HttpPostConfiguration;

use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = ".callmemaybe";
const ERR_NO_HOME_PATH: &str = "Could not find the home directory, or it does not exist";
const ERR_NO_CONFIG_DFLT_PATH: &str = "Could not find ~/.callmemaybe nor ~/.callmemaybe.toml";
const DEFAULT_HOSTNAME: &str = "{UNK HOST}";
const DEFAULT_TITLE_FORMAT: &str = "$host %a-%b-%Y";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub title_fmt: Option<String>,
    pub pushover: Option<PushOverConfiguration>,
    #[cfg(feature = "libinotify")]
    pub libnotify: Option<LibNotifyConfiguration>,
    pub http_post: Option<HttpPostConfiguration>
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
    /// Generate the title according to the pattern specified in the config.
    /// If no pattern is configured, a default pattern (the current date) is
    /// returned.
    ///
    /// The following placeholders are supported:
    /// All the strftime formats from chrono
    /// $host for the hostname
    pub fn generate_title(&self) -> String {
        let now = chrono::offset::Local::now();
        let fmt = self
            .title_fmt
            .as_ref()
            .map_or(DEFAULT_TITLE_FORMAT, |fmt| &fmt[..]);

        let maybe_hostname = hostname::get();
        let hostname = maybe_hostname
            .as_ref()
            .map(|host_os_str| { host_os_str.to_str() }.unwrap_or(DEFAULT_HOSTNAME))
            .unwrap_or(DEFAULT_HOSTNAME);

        let wo_host = now.format(fmt).to_string();

        wo_host.replace("$host", hostname)
    }

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
