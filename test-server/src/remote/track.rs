use crate::{error::Error, remote::RemoteServer, traits::track::TracksAPI};

impl TracksAPI for RemoteServer {
    fn get_tracks(&self) -> crate::error::Result<Vec<String>> {
        Err(Error::ActionNotImplemented)
    }
}