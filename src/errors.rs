use std::io;

pub enum CommandError {
    #[allow(dead_code)]
    IoError(io::Error),
    ExitCommand,
}

impl From<io::Error> for CommandError {
    fn from(e: io::Error) -> CommandError {
        CommandError::IoError(e)
    }
}
