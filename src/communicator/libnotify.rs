use super::{Communicator, CommunicatorPriority, Message};
use notify_rust::Notification;
use serde::Deserialize;

const NAME: &str = "libnotify";

#[derive(Deserialize, Debug)]
pub struct LibNotifyConfiguration {
    pub priority: Option<i32>,
}

impl Communicator for LibNotifyConfiguration {
    fn send(&self, message: &Message, _ : &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        Notification::new()
            .summary(&message.title)
            .body(&message.contents)
            .show()?;

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
