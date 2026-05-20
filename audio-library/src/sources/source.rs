use crate::{error::Error, track::Track};

// TODO: make "pub(crate)"
pub trait Source {
    fn get_tracks(&self) -> impl Iterator<Item = SourceTrackResult>;
}

pub trait SourceOptions: Default + Clone {}

pub enum SourceTrackResult {
    Track(Track),
    PartialTrack(Track, Vec<Error>),
    Error(Error)
}