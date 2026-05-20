use std::{collections::HashMap};

use crate::{library::source::LibrarySource, track::Track};

pub type TrackLibraryID = u32; 

pub struct Library {
    pub tracks: HashMap<TrackLibraryID, Track>,
}

impl Library {
    pub fn new(source: LibrarySource) {
        todo!()
    }
}