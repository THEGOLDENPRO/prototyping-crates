use std::{collections::HashMap};

use crate::{library::source::LibrarySource, track::Track};

pub type TrackLibraryID = u32; 

pub struct Library {
    pub tracks: HashMap<TrackLibraryID, Track>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            tracks: HashMap::new()
        }
    }

    pub fn add_source(&mut self, source: LibrarySource) {
        todo!()
    }
}