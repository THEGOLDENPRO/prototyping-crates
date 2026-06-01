use std::{fmt::Display, result::Result as StdResult};
use audio_library::{error::Error as AudioLibraryError};

use crate::{event::Event, library::LibraryTrackID};

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug)]
pub enum Error {
    LibraryIsFull,

    GatewaySendFailure { event: Event, error: String },

    ActionNotImplemented,

    // internal crates
    AudioLibrary(AudioLibraryError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::LibraryIsFull => write!(
                f,
                "Library has too many tracks! (the limit on this system is {})",
                LibraryTrackID::MAX
            ),
            Error::GatewaySendFailure { event, error } => write!(
                f,
                "Failed to send '{:?}' event to gateway! Error: {}",
                event, error
            ),
            Error::ActionNotImplemented => write!(
                f,
                "This action is not implemented in this server mode!"
            ),
            Error::AudioLibrary(audio_library_error) => write!(
                f,
                "{audio_library_error}"
            ),
        }
    }
}

impl From<AudioLibraryError> for Error {
    fn from(value: AudioLibraryError) -> Self {
        Error::AudioLibrary(value)
    }
}