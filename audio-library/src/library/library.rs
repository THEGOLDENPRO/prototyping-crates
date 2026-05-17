use std::{collections::HashMap};

use crate::{library::source::LibrarySource, track::Track};

pub type TrackID = u32; 

pub struct Library {
    pub tracks: HashMap<TrackID, Track>
}

impl Library {
    pub fn new(source: LibrarySource) {
        todo!()
    }
}