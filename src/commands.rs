use crate::utils::parse_cmd;

#[derive(Debug)]
pub enum Command {
    Echo(String),
    Exit,
    NotFound(String),
}

impl From<String> for Command {
    fn from(input: String) -> Self {
        let (cmd, value) = parse_cmd(input);
        match cmd.as_str() {
            "exit" => Command::Exit,
            _ => Command::NotFound(cmd.to_string()),
        }
    }
}
