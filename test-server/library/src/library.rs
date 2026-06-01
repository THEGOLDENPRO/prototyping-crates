use std::collections::HashMap;

use audio_library::{error::Error as AudioLibraryError, track::Track};

use crate::error::{Error, Result};

pub type LibrarySourceID = usize;

#[cfg(not(feature = "force-u64-library"))]
pub type LibraryTrackID = usize;

#[cfg(feature = "force-u64-library")]
pub type LibraryTrackID = u64;

// TODO: look into switching to FxHashMap
#[derive(Default)]
pub struct Library {
    pub tracks: HashMap<LibraryTrackID, Track>,

    pub track_sources: HashMap<LibraryTrackID, LibrarySourceID>,

    pub track_count: LibraryTrackID,
}

impl Library {
    pub fn add_track(&mut self, track: Track, source_id: LibrarySourceID) -> Result<LibraryTrackID> {
        // TODO: add source to library with ID and vector of tracks that source has.

        if let Some(next_track_id) = self.track_count.checked_add(1) {
            self.tracks.insert(next_track_id, track);
            self.track_sources.insert(next_track_id, source_id);

            return Ok(next_track_id)
        }

        // imagine having more than 18,446,744,073,709,551,615 tracks 💀

        return Err(Error::LibraryIsFull)
    }
}