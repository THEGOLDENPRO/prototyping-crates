use crate::sources::result::SourceTrackResult;

// TODO: make "pub(crate)"
pub trait Source {}

pub trait SourceOptions: Default + Clone {}

// TODO: make "pub(crate)"
pub trait SourceTrack {
    fn get_tracks(&self) -> impl Iterator<Item = SourceTrackResult>;
}