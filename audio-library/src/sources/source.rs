use crate::{error::Error, track::Track};

// TODO: make "pub(crate)"
pub trait Source {}

pub trait SourceOptions: Default + Clone {}

// TODO: make "pub(crate)"
pub trait SourceTrack {
    fn get_tracks(&self) -> impl Iterator<Item = SourceTrackResult>;
}

pub enum SourceTrackResult {
    Track(Track),
    PartialTrack(Track, Vec<Error>),
    Error(Error)
}