use std::io::{stdin};
use std::process;

pub fn brisk_repl() {
    let mut user_input = String::new();
    println!("Welcome to the brisk interactive editor");
    stdin().read_line(&mut user_input).expect("Unexpected error, invalid input detected");
    while true {
        print!(">");
        stdin().read_line(&mut user_input).expect("Unexpected error, invalid input detected");
        if user_input == "exit" || user_input.is_empty() {
            break
        }
    }
    process::exit(0)
}