use crate::{error::Error, track::Track};

pub enum SourceTrackResult {
    Track(Track),
    PartialTrack(Track, Vec<Error>),
    Error(Error)
}