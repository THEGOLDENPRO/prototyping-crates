use crate::library::LibraryTrackID;

#[derive(Debug)]
pub enum Event {
    TrackAdded(LibraryTrackID)
}