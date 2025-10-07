use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("std::io error")]
    StdIoError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ListenerError {
    #[error("connection error")]
    ConnectionError,

    #[error("user has not identified yet")]
    UserIsUnidentified,
}

#[derive(Error, Debug)]
pub enum SenderError {
    #[error("std::io error")]
    StdIoError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum CommandExecError {
    #[error("command does not exist")]
    NonexistantCommand,
}

// Conversion impls here
impl From<SenderError> for ListenerError {
    fn from(value: SenderError) -> Self {
        match value {
            SenderError::StdIoError(_) => Self::ConnectionError,
        }
    }
}

impl From<std::io::Error> for ListenerError {
    fn from(_: std::io::Error) -> Self {
        Self::ConnectionError
    }
}
