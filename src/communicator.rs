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

    match a_prio {
        CommunicatorPriority::Default => match b_prio {
            Default => Ordering::Equal,
            _ => Ordering::Less,
        },
        CommunicatorPriority::Priority(a_prio_n) => match b_prio {
            CommunicatorPriority::Default => Ordering::Greater,
            CommunicatorPriority::Priority(b_prio_n) => a_prio_n.cmp(&b_prio_n),
        },
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
