use crate::commands::Command;
use std::io::{Write, stdin, stdout};

pub fn print_error(input: String) {
    eprintln!("{:?}: command not found", input)
}

// fn get_cmd(input: String) -> Result<Command, String> {
//     Command::new
// }

pub fn get_user_input() -> Result<String, String> {
    print!("$ ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    Ok(input.trim().to_string())
}

pub fn parse_cmd(input: String) -> (String, Vec<String>) {
    match input.split_once(" ") {
        Some((cmd, arguments)) => {
            let args = arguments.split_whitespace().map(str::to_string).collect();
            (String::from(cmd), args)
        }
        None => (input, vec![]),
    }
}

pub fn check_if_builtin(args: Vec<String>) {
    for arg in args {
        let to_print = match Command::from((arg.clone(), vec![])) {
            Command::NotFound(_) => {
                format!("{}: not found", arg)
            }
            _ => {
                format!("{} is a shell builtin", arg)
            }
        };
        println!("{to_print}");
    }
}
