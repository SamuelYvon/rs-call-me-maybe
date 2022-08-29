pub mod pushover;
use crate::config::Config;

pub struct Message {
    pub title : String,
    pub contents : String,
}

pub trait Communicator {

    fn send(&self, message : &Message) -> Result<(), Box<dyn std::error::Error>>;

}


pub fn resolve(config : Config) -> Option<Box<dyn Communicator>> {

    if let Some(po_config) = config.pushover {
        return Some(Box::new(po_config));
    }


    None
}
