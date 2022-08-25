pub mod pushover;

pub struct Message {
    pub title : String,
    pub contents : String,
}


pub trait Communicator {

    fn send(&self, message : &Message) -> Result<(), Box<dyn std::error::Error>>;

}


