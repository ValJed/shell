use crate::commands::Command;
use std::fs::{self, Metadata};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::{
    env,
    io::{Write, stdin, stdout},
};

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

pub fn check_type(args: Vec<String>) {
    for arg in args {
        let to_print = match Command::from((arg.clone(), vec![])) {
            Command::NotFound(cmd) => match search_path_bin(cmd) {
                Some(path) => {
                    format!("{} is {:?}", arg, path)
                }
                None => {
                    format!("{}: not found", arg)
                }
            },
            _ => {
                format!("{} is a shell builtin", arg)
            }
        };
        println!("{to_print}");
    }
}

pub fn search_path_bin(cmd: String) -> Option<PathBuf> {
    match env::var("PATH") {
        Ok(val) => {
            let path = val.split(":").find(|p| {
                let dir = fs::read_dir(p);
                if dir.is_err() {
                    return false;
                }
                for file_res in dir.unwrap() {
                    let Ok(file) = file_res else {
                        continue;
                    };
                    let name = file.file_name().to_owned();
                    if name != cmd.as_str() {
                        continue;
                    }
                    match file.metadata() {
                        Ok(metadata) => return check_file_execute_permission(metadata),
                        Err(_) => {}
                    };
                }

                return false;
            })?;

            Some(PathBuf::from(format!("{path}/{cmd}")))
        }
        Err(_) => None,
    }
}

fn check_file_execute_permission(metadata: Metadata) -> bool {
    let mode = metadata.permissions().mode();

    let owner_exec = mode & 0o100 != 0; // user/owner can execute
    let group_exec = mode & 0o010 != 0; // group can execute
    let others_exec = mode & 0o001 != 0; // others can execute
    //
    if owner_exec || group_exec || others_exec {
        return true;
    } else {
        return false;
    }
}
