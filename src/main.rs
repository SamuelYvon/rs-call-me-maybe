use std::io;
use Result::Err;

mod communicator;
mod config;

use clap::Parser;
use communicator::{resolve, Communicator, Message};
use config::Config;
use simple_error::bail;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Only parse the config, but do not execute
    #[clap(short, long, takes_value = false)]
    parse_config: bool,

    #[clap(short, long, takes_value=false)]
    list_communicators : bool,

    /// alternative configuration path. By default, uses the ~/.callmemaybe[.toml] file
    #[clap(long)]
    config: Option<PathBuf>,

    /// The title to use for the message
    #[clap(short, long)]
    title : Option<String>,

    /// specify a communicator to use by name
    #[clap(short, long)]
    communicator: Option<String>,

    /// specify parameters that can be passed to the communicator
    #[clap(index = 1)]
    positional_arguments: Vec<String>,
}

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

    lines
}

fn stdin_input(title : String) -> Message {
    // TODO: maybe other ways of getting STDIN
    let contents = read_stdin().join("\n");
    Message {
        title,
        contents,
    }
}

fn use_exact <F: FnOnce() -> Message>(
    name: &str,
    communicators: Vec<&dyn Communicator>,
    input : F,
    arguments : &[&str]
) -> Result<(), Box<dyn std::error::Error>> {

    let of_name = communicators.into_iter().find(|&e| e.name() == name);
    let communicator = of_name.ok_or(format!("No communicator with the provided name ({name})"))?;
    let message = input();

    communicator.send(&message, arguments)?;

    Ok(())
}

fn use_first_working<F: FnOnce() -> Message>(
    communicators: Vec<&dyn Communicator>,
    input : F,
    arguments : &[&str]
) -> Result<(), Box<dyn std::error::Error>> {
    let mut errors: Vec<Box<dyn std::error::Error>> = Vec::new();

    let message = input();
    let communicator_sz = communicators.len();

    for communicator in communicators.into_iter() {
        match communicator.send(&message, arguments) {
            Ok(()) => break,
            Err(e) => errors.push(e),
        };
    }

    return if errors.len() < communicator_sz {
        Ok(())
    } else {
        let error_list: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        let error_list_formatted = error_list.join("\n-");
        Err(format!("Could not invoke any communicator. Here is the list of errors:\n-{error_list_formatted}"))?
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = Config::resolve(args.config)?;

    if args.parse_config {
        println!("Config is OK");
        return Ok(());
    }

    let communicators = resolve(&config);

    if args.list_communicators { 
        println!("Communicators available: ");
        for comm in &communicators {
            println!("\t-{}", comm.name());
        }
        return Ok(());
    }

    if communicators.is_empty() {
        bail!("No communicator found, cannot proceed.");
    }

    let input = || {
        stdin_input(args.title.unwrap_or_else(|| config.generate_title()))
    };

    let arg_reg_vec : Vec<&str> = args.positional_arguments.iter().map(AsRef::as_ref).collect();
    let arguments = arg_reg_vec.as_slice();

    if let Some(communicator_name) = args.communicator {
        use_exact(&communicator_name, communicators, input, arguments)
    } else {
        use_first_working(communicators, input, arguments)
    }
}
