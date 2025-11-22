#[allow(unused_imports)]
use std::io::{self, Write};

enum Command {
    Exit,
    Echo { display_string: String },
    NotFound,
}

impl Command {
    fn from_input(input: &str) -> Self {
        let input = input.trim();
        if input == "exit" {
            return Self::Exit;
        }
        if let Some(idx) = input.find("echo ") {
            if idx == 0 {
                return Self::Echo {
                    display_string: input["echo ".len()..].to_string(),
                };
            }
        }
        Self::NotFound
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
            Command::NotFound => println!("{}: command not found", input.trim()),
        }
    }
}
