use std::io;
use Result::Err;

mod communicator;
mod config;

use config::Config;
use communicator::{resolve, Communicator, Message};
use simple_error::bail;

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();

    loop {
        let mut line = String::new();

        match stdin.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                lines.push(line);
            }
            Err(e) => panic!("{e}"),
        }
    }

    return lines;
}

fn read_and_send(client: Box<dyn Communicator>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = read_stdin().join("\n");

    let message = Message {
        title: "hello".to_string(),
        contents: contents,
    };

    client.send(&message)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::resolve()?;
    let communicator = resolve(config);

    match communicator {
        None => {
            bail!("No communicator found")
        }
        Some(client) => read_and_send(client),
    }
}
