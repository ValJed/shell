use commands::Command as Cmd;
use std::process::{self, Command};
use utils::{get_user_input, print_error};

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
        match Cmd::from(input) {
            Cmd::Exit => {
                process::exit(1);
            }
            Cmd::NotFound(cmd) => print_error(cmd),
            Cmd::Echo(cmd) => {}
        }

        // print_error(cmd);
    }
}
