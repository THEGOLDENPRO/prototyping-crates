use crate::error::Result;

pub trait TracksAPI {
    // string is placeholder
    fn get_tracks(&self) -> Result<Vec<String>>;
}