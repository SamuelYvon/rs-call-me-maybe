use std::io;
use Result::Err;

mod communicator;
mod config;

use communicator::{resolve, Message};
use config::Config;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::resolve()?;
    let communicators = resolve(config);

    if 0 == communicators.len() {
        bail!("No communicator found");
    }

    let mut errors: Vec<Box<dyn std::error::Error>> = Vec::new();

    // TODO: maybe other ways of getting STDIN
    let contents = read_stdin().join("\n");
    let message = Message {
        title: "hello".to_string(),
        contents: contents,
    };

    for communicator in communicators.iter() {
        match communicator.send(&message) {
            Ok(()) => break,
            Err(e) => errors.push(e),
        };
    }

    return if errors.len() < communicators.len() {
        Ok(())
    } else {
        let error_list: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        let error_list_formatted = error_list.join("\n-");
        Err(format!("Could not invoke any communicator. Here is the list of errors:\n-{error_list_formatted}"))?
    };
}
