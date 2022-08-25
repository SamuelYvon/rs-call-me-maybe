use std::io;
use Result::Err;

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
    for line in read_stdin().iter() {
        print!("{line}");
    }
}
