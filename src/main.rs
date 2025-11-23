#[allow(unused_imports)]
use std::io::{self, Write};

const BUILT_IN_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

enum Command {
    Exit,
    Echo { display_string: String },
    Type { cmd_name: String },
    NotFound { cmd_not_found: String },
}

impl Command {
    fn from_input(input: &str) -> Self {
        let trimmed = input.trim();

        match trimmed {
            "exit" => Command::Exit,
            _ if trimmed.starts_with("echo ") => Command::Echo {
                display_string: trimmed.strip_prefix("echo ").unwrap().to_string(),
            },
            _ if trimmed.starts_with("type ") => Command::Type {
                cmd_name: trimmed.strip_prefix("type ").unwrap().to_string(),
            },
            _ => Command::NotFound {
                cmd_not_found: trimmed.to_string(),
            },
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = Command::from_input(&input);

        match command {
            Command::Exit => std::process::exit(0),
            Command::Echo { display_string } => println!("{}", display_string),
            Command::Type { cmd_name } => {
                if BUILT_IN_COMMANDS.contains(&cmd_name.as_str()) {
                    println!("{} is a shell builtin", cmd_name)
                } else {
                    println!("{}: command not found", cmd_name)
                }
            }
            Command::NotFound { cmd_not_found } => println!("{}: command not found", cmd_not_found),
        }
    }
}
