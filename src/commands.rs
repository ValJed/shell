#[derive(Debug)]
pub enum Command {
    Echo(Vec<String>),
    Exit,
    NotFound(String),
    Type(Vec<String>),
}

impl From<(String, Vec<String>)> for Command {
    fn from((cmd, args): (String, Vec<String>)) -> Self {
        match cmd.as_str() {
            "exit" => Command::Exit,
            "echo" => Command::Echo(args),
            "type" => Command::Type(args),
            _ => Command::NotFound(cmd),
        }
    }
}
