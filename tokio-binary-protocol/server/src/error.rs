use std::{fmt::Display, result::Result as StdResult};

pub type Result<T, E = Error> = StdResult<T, E>;

pub enum Error {
    TCPListenerBindFailure { error: String },
    TCPListenerAcceptConnectionsFailure { error: String }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TCPListenerBindFailure { error } => write!(
                f, "Failed to create TCP listener! Error: {error}"
            ),
            Error::TCPListenerAcceptConnectionsFailure { error } => write!(
                f, "Failed to accept TCP connections! Error: {error}"
            ),
        }
    }
}