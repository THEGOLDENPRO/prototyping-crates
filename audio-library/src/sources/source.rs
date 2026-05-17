use crate::{error::Error, track::Track};

pub trait Source { // TODO: make "pub(crate)"
    type TrackIterator;

    fn get_tracks(&self) -> Self::TrackIterator;
}

pub enum SourceTrackResult {
    Track(Track),
    PartialTrack(Track, Vec<Error>),
    Error(Error)
}