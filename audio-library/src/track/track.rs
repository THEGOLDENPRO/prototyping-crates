use std::io::BufReader;

use crate::track::{issues::TrackIssues, metadata::Metadata, reader::TrackReader};

pub struct Track {
    pub metadata: Metadata,
    pub buf_reader: BufReader<TrackReader>,

    pub issues: Vec<TrackIssues>,
}

impl Track {}