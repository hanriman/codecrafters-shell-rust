#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // wait for user input
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap_or(404);
    println!("{}: command not found", command.trim());
}
