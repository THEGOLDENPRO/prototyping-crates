use std::{fmt::Display, result::Result as StdResult};

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug)]
pub enum Error {
    RecursiveWalkFailure { error: String },
    FileOpenFailure { file_name: String, error: String },
    FileFormatReadHeaderFailure { file_name: String, error: String },
    AudioTagsParseFailure { audio_file_name: String, error: String },
    AudioFormatExtensionParseFailure { audio_file_name: String, format: String },
    UnknownFileFormat { file_name: String },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RecursiveWalkFailure { error } => write!(
                f,
                "Error during recursively walking over directory! Error: {error}"
            ),
            Error::FileOpenFailure { file_name, error } => write!(
                f,
                "Failed to open file '{file_name}'! Error: {error}"
            ),
            Error::FileFormatReadHeaderFailure { file_name, error } => write!(
                f,
                "Failed to read the file ('{file_name}') \
                    header to get file format! Error: {error}"
            ),
            Error::AudioTagsParseFailure { audio_file_name, error } => write!(
                f,
                "Failed to extract tags from the audio file '{audio_file_name}'! \
                    This file will lack metadata! Error: {error}"
            ),
            Error::AudioFormatExtensionParseFailure { audio_file_name, format } => write!(
                f,
                "Could not parse audio file format \
                    extension ('{format}') for '{audio_file_name}'!"
            ),
            Error::UnknownFileFormat { file_name } => write!(
                f, "Cannot detect file format of the file '{file_name}'!"
            )
        }
    }
}