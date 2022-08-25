use std::io;
use std::collections::HashMap;
use Result::Err;

const PUSH_OVER_URL : &str = "https://api.pushover.net/1/messages.json";

fn pusherover(message : String) -> Result<(), Box<dyn std::error::Error>> {
    let mut json_data : HashMap<&str, String> = HashMap::new();

    json_data.insert("token", "".to_string());
    json_data.insert("user", "".to_string());
    json_data.insert("message", message);

    let client = reqwest::blocking::Client::new();
    let _ = client.post(PUSH_OVER_URL)
        .json(&json_data)
        .send();


    Ok(())
}

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

fn main() {
    let message = read_stdin().join("\n");
    pusherover(message);
}
