pub enum Command {
    List,
}

impl Command {
    pub fn from_string(to_convert: &str) -> Option<Command> {
        if to_convert == String::from("ls") {
            return Some(Command::List)
        }

        None
    }
}