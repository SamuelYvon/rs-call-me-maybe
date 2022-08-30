use serde::{Deserialize};
use super::{Communicator, Message};
use std::collections::HashMap;

const PUSH_OVER_URL: &str = "https://api.pushover.net/1/messages.json";
const APP_TOKEN_PARAM: &str = "token";
const USER_TOKEN_PARAM: &str = "user";
const MESSAGE_PARAM: &str = "message";

#[derive(Deserialize)]
#[derive(Debug)]
pub struct PushOverConfiguration {
    pub app_token : String,
    pub user_token : String
}

impl Communicator for PushOverConfiguration {
    fn send(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let mut json_data: HashMap<&str, &str> = HashMap::new();

        json_data.insert(APP_TOKEN_PARAM, &self.app_token);
        json_data.insert(USER_TOKEN_PARAM, &self.user_token);
        json_data.insert(MESSAGE_PARAM, &message.contents);

        let client = reqwest::blocking::Client::new();
        client.post(PUSH_OVER_URL).json(&json_data).send()?;

        Ok(())
    }
}
