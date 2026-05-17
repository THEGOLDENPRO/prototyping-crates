use std::{fs::File, io::{Read, Seek}};

pub enum TrackReader {
    File(File),
    // Network(Cursor<Vec<u8>>), 
}

impl Read for TrackReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            TrackReader::File(file) => file.read(buf),
        }
    }
}

impl Seek for TrackReader {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match self {
            TrackReader::File(file) => file.seek(pos),
        }
    }
}
