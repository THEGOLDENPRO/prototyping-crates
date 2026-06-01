use crate::{error::Result, state::{ServerState}};

/// Trait that holds all the actions you can perform on a server.
pub trait Server {
    /// Returns latest cached server library (tracks, artists, album art, etc) and other state.
    /// 
    /// This function will never do any processing or fetching, it 
    /// just returns data that the server already has as fast as possible.
    fn get_state(&self) -> &ServerState;

    fn load_tracks(&mut self) -> Result<()>;
}