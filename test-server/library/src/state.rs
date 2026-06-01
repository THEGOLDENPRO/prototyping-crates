use crate::{error::Error, library::Library};

// TODO: might need to be renamed to "Snapshot" and have 
// get_state return an owned version of this with Library and errors 
// as Arc and Mutex for thread safety but we'll see about that.
#[derive(Default)]
pub struct ServerState {
    pub library: Library,

    pub errors: Vec<Error>
}