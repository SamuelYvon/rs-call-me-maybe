use super::{Communicator, CommunicatorPriority, Message};
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

const NAME: &str = "httppost";
const TITLE_PARAM: &str = "title";
const MESSAGE_PARAM: &str = "message";
const ERR_NO_URL: &str = "No URL provided, cannot send the request";
const ERR_NOT_AN_URL: &str =
    "The first argument provided is not an URL; requires a valid URL to send a post message";

#[derive(Deserialize, Debug)]
pub struct HttpPostConfiguration {
    pub priority: Option<i32>,
}

impl Communicator for HttpPostConfiguration {
    fn send(&self, message: &Message, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let n = args.len();

        if 0 == n {
            Err(ERR_NO_URL)?;
        }

        let raw_url = args[0];

        if Url::parse(raw_url).is_err() {
            Err(ERR_NOT_AN_URL)?;
        }

        let mut json_data: HashMap<&str, &str> = HashMap::new();
        json_data.insert(TITLE_PARAM, &message.title);
        json_data.insert(MESSAGE_PARAM, &message.contents);

        let client = reqwest::blocking::Client::new();
        client.post(raw_url).json(&json_data).send()?;

        Ok(())
    }

    fn priority(&self) -> CommunicatorPriority {
        match self.priority {
            Some(p) => CommunicatorPriority::Priority(p),
            None => CommunicatorPriority::Default,
        }
    }

    fn name(&self) -> &'static str {
        NAME
    }
}
