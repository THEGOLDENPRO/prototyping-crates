use std::io::{Read, Seek};

use crate::track::Track;

pub trait Source {
    // NOTE: for now a vector for testing 
    // but this will very likely change.

    /// Get and construct tracks from source.
    fn get_tracks<R: Read + Seek>(&self) -> Vec<Track<R>>;
}