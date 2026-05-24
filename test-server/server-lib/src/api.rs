use crate::error::Result;

pub trait ServerAPI: TracksAPI {}

pub trait TracksAPI {
    // string is placeholder
    fn get_tracks(&self) -> Result<Vec<String>>;
}