pub mod pushover;

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
        return CommunicatorPriority::Default;
    }

    fn send(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>>;
}

fn communicator_cmp(a: &Box<dyn Communicator>, b: &Box<dyn Communicator>) -> Ordering {
    let a_prio = a.priority();
    let b_prio = b.priority();

    match (a_prio, b_prio) {
        (CommunicatorPriority::Default, CommunicatorPriority::Default) => Ordering::Equal,
        (CommunicatorPriority::Default, CommunicatorPriority::Priority(_)) => Ordering::Less,
        (CommunicatorPriority::Priority(_), CommunicatorPriority::Default) => Ordering::Greater,
        (CommunicatorPriority::Priority(a), CommunicatorPriority::Priority(b)) => a.cmp(&b)
    }
}

pub fn resolve(config: Config) -> Vec<Box<dyn Communicator>> {
    let mut vector: Vec<Box<dyn Communicator>> = Vec::new();

    if let Some(po_config) = config.pushover {
        vector.push(Box::new(po_config));
    }

    vector.sort_by(communicator_cmp);

    return vector;
}
