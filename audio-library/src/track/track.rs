use std::io::{BufReader, Read, Seek};

use crate::track::metadata::Metadata;

// TODO: get track title and cover

pub struct Track<R: Read + Seek> {
    pub metadata: Metadata,
    pub buf_reader: BufReader<R>,
}

impl<R: Read + Seek> Track<R> {}