use std::io::BufReader;

use crate::track::{metadata::Metadata, reader::TrackReader};

pub struct Track {
    pub metadata: Metadata,
    pub buf_reader: BufReader<TrackReader>,
}

impl Track {}