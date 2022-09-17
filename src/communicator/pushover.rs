use super::{Communicator, CommunicatorPriority, Message};
use serde::Deserialize;
use std::collections::HashMap;

const PUSH_OVER_URL: &str = "https://api.pushover.net/1/messages.json";
const APP_TOKEN_PARAM: &str = "token";
const USER_TOKEN_PARAM: &str = "user";
const MESSAGE_PARAM: &str = "message";
const TITLE_PARAM : &str = "title";
const NAME: &str = "pushover";

#[derive(Deserialize, Debug)]
pub struct PushOverConfiguration {
    pub app_token: String,
    pub user_token: String,
    pub priority: Option<i32>,
}

impl Communicator for PushOverConfiguration {
    fn send(&self, message: &Message, _ : &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let mut json_data: HashMap<&str, &str> = HashMap::new();

        json_data.insert(APP_TOKEN_PARAM, &self.app_token);
        json_data.insert(USER_TOKEN_PARAM, &self.user_token);
        json_data.insert(MESSAGE_PARAM, &message.contents);
        json_data.insert(TITLE_PARAM, &message.title);

        let client = reqwest::blocking::Client::new();
        client.post(PUSH_OVER_URL).json(&json_data).send()?;

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
