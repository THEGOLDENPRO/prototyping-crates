use crate::{error::Result, sources::r#type::SourceType, track::Track};

// TODO: make "pub(crate)"
pub trait Source {
    fn get_type(&self) -> SourceType;
    // fn get_metadata(&self) -> SourceMetadata;
    fn get_tracks(&self) -> impl TrackIterator;
}

pub trait SourceOptions: Default + Clone {}

pub trait TrackIterator: Iterator<Item = Result<Track>> {}