use crate::{api::TracksAPI, error::Error, remote::RemoteServer};

impl TracksAPI for RemoteServer {
    fn get_tracks(&self) -> crate::error::Result<Vec<String>> {
        Err(Error::ActionNotImplemented)
    }
}