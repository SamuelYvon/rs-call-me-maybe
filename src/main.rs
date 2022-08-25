use std::io;
use Result::Err;

mod communicator;
use communicator::{Message, Communicator};
use communicator::pushover::PushOverClient;

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines : Vec<String> = Vec::new();

    loop {
        let mut line = String::new();

        match stdin.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                lines.push(line);
            },
            Err(e) => panic!("{e}")
        }
    }

    return lines;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = read_stdin().join("\n");

    let message = Message {
        title: "hello".to_string(),
        contents : contents
    };

    let client = PushOverClient::new();
    client.send(&message)?;

    Ok(())
}
