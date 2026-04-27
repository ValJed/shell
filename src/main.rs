use commands::Command as Cmd;
use std::{
    io::{self, Write},
    process::{self},
};
use utils::{check_type, get_user_input, parse_cmd, print_error};

mod commands;
mod utils;

fn main() {
    loop {
        let input_res = get_user_input();
        if input_res.is_err() {
            eprintln!("{}", input_res.unwrap_err());
            process::exit(126);
        }

        let input = input_res.unwrap();
        let (command, arguments) = parse_cmd(input);
        match Cmd::from((command, arguments)) {
            Cmd::NotFound(cmd) => print_error(cmd),
            Cmd::Exit => {
                process::exit(1);
            }
            Cmd::Echo(args) => {
                print!("{:?}", args);
            }
            Cmd::Type(args) => check_type(args),
        }
    }
}
