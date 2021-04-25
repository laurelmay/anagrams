use std::io;

pub enum CommandError {
    IoError(io::Error),
    ExitCommand,
}

impl From<io::Error> for CommandError {
    fn from(e: io::Error) -> CommandError {
        CommandError::IoError(e)
    }
}
