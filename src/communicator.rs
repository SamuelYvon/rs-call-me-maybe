pub mod httppost;
#[cfg(feature = "libinotify")]
pub mod libnotify;
pub mod pushover;

use crate::config::Config;
use std::cmp::Ordering;

use httppost::HttpPostConfiguration;

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

    fn send(&self, message: &Message, arguments: &[&str])
        -> Result<(), Box<dyn std::error::Error>>;

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
        (CommunicatorPriority::Priority(a), CommunicatorPriority::Priority(b)) => b.cmp(&a),
    }
}

pub fn resolve(config: &Config) -> Vec<&dyn Communicator> {
    let mut vector: Vec<&dyn Communicator> = Vec::new();

    if let Some(po_config) = &config.pushover {
        vector.push(po_config);
    }

    #[cfg(feature = "libinotify")]
    {
        if let Some(lib_ntfy) = &config.libnotify {
            vector.push(lib_ntfy);
        }
    }

    vector.push(match &config.http_post {
        Some(post_config) => post_config,
        _ => &HttpPostConfiguration { priority: None },
    });

    vector.sort_by(communicator_cmp);

    vector
}
