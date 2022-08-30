pub mod pushover;
use crate::config::Config;

pub struct Message {
    pub title : String,
    pub contents : String,
}

pub trait Communicator {

    fn send(&self, message : &Message) -> Result<(), Box<dyn std::error::Error>>;

}


pub fn resolve(config : Config) -> Vec<Box<dyn Communicator>> {
    let mut vector : Vec<Box<dyn Communicator>> = Vec::new();

    if let Some(po_config) = config.pushover {
        vector.push(Box::new(po_config));
    }


    return vector;
}
