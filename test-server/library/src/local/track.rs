use crate::{error::Error, local::LocalServer, api::TracksAPI};

impl TracksAPI for LocalServer {
    fn get_tracks(&self) -> crate::error::Result<Vec<String>> {
        Err(Error::ActionNotImplemented)
    }
}