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

pub fn parse_cmd(input: String) -> (String, String) {
    match input.split_once(" ") {
        Some((cmd, val)) => (String::from(cmd), String::from(val)),
        None => (input, String::new()),
    }
}
