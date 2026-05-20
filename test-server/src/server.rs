use std::sync::mpsc::{self, Receiver, Sender};

use crate::{api::ServerAPI, event::Event};

pub struct Server<InnerServer: ServerAPI> {
    server: InnerServer,

    pub gateway: (Sender<Event>, Receiver<Event>),
}

impl<InnerServer: ServerAPI> Server<InnerServer> {
    pub fn new(server: InnerServer) -> Self {
        let (sender, receiver) = mpsc::channel();

        Self {
            server,
            gateway: (sender, receiver)
        }
    }
}