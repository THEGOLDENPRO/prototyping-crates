use crate::{error::Error, local::LocalServer, traits::track::TracksAPI};

impl TracksAPI for LocalServer {
    fn get_tracks(&self) -> crate::error::Result<Vec<String>> {
        Err(Error::ActionNotImplemented)
    }
}