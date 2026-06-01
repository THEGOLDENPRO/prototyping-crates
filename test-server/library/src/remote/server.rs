use crate::{error::{Error, Result}, state::ServerState, traits::Server};

pub struct RemoteServer {
    state: ServerState
}

impl Server for RemoteServer {
    fn get_state(&self) -> &ServerState {
        &self.state
    }

    fn load_tracks(&mut self) -> Result<()> {
        Err(Error::ActionNotImplemented)
    }
}