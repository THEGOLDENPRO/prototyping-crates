use std::collections::HashMap;

use audio_library::{sources::traits::Source, track::Track};

pub type LibraryTrackID = u32;

pub struct Library {
    pub sources: HashMap<String, Box<dyn Source>>,
    pub tracks: HashMap<LibraryTrackID, Track>,
}