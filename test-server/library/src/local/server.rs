use std::collections::HashMap ;

use audio_library::{sources::traits::Source};

use crate::{error::Result, library::{Library, LibrarySourceID}, state::ServerState, traits::Server};

pub struct LocalServer<S: Source> {
    state: ServerState,

    // TODO: document the usage of usize for source id
    sources: HashMap<LibrarySourceID, S>,

    // NOTE: we will need some unified way to send 
    // and listen to events as RemoteServer will 
    // not have an mpsc based gateway.
    // pub gateway: (Sender<Event>, Receiver<Event>),
}

impl<S: Source> LocalServer<S> {
    pub fn new(sources: Vec<S>) -> Self {
        let mut mapped_sources = HashMap::new();

        for (source_id, source) in sources.into_iter().enumerate() {
            mapped_sources.insert(source_id, source);
        }

        Self {
            state: ServerState::default(),

            sources: mapped_sources,
        }
    }
}

impl<S: Source> Server for LocalServer<S> {
    fn get_state(&self) -> &ServerState {
        &self.state
    }

    fn load_tracks(&mut self) -> Result<()> {
        for (source_id, source) in &self.sources {
            for track_result in source.get_tracks() {
                match track_result {
                    Ok(track) => self.state.library.add_track(track, source_id.to_owned())?,
                    Err(error) => {
                        self.state.errors.push(error.into());
                        continue;
                    },
                };
            }
        }

        Ok(())
    }
}