pub mod pushover;
#[cfg(feature = "libinotify")]
pub mod libnotify;

use crate::config::Config;
use std::cmp::Ordering;

pub enum CommunicatorPriority {
    Priority(i32),
    Default,
}

pub struct Message {
    pub title: String,
    pub contents: String,
}

pub trait Communicator {
    fn priority(&self) -> CommunicatorPriority {
        CommunicatorPriority::Default
    }

    fn send(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>>;

    fn name(&self) -> &'static str;
}

/// Compare two communicators based on priority. Higher priority value will have
/// precedence over lower priority number values. The default value is the lowest
/// possible value.
fn communicator_cmp(a: &&dyn Communicator, b: &&dyn Communicator) -> Ordering {
    let a_prio = a.priority();
    let b_prio = b.priority();

    match (a_prio, b_prio) {
        (CommunicatorPriority::Default, CommunicatorPriority::Default) => Ordering::Equal,
        (CommunicatorPriority::Default, CommunicatorPriority::Priority(_)) => Ordering::Greater,
        (CommunicatorPriority::Priority(_), CommunicatorPriority::Default) => Ordering::Less,
        (CommunicatorPriority::Priority(a), CommunicatorPriority::Priority(b)) => b.cmp(&a)
    }
}

pub fn resolve(config: &Config) -> Vec<&dyn Communicator> {
    let mut vector: Vec<&dyn Communicator> = Vec::new();

    if let Some(po_config) = &config.pushover {
        vector.push(po_config);
    }

    #[cfg(feature = "libinotify")] {
        if let Some(lib_ntfy) = &config.libnotify {
            vector.push(lib_ntfy);
        }
    }

    vector.sort_by(communicator_cmp);

    vector
}
