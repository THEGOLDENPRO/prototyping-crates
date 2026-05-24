#[cfg(test)]
mod tests {
    use std::thread;

use test_server_lib::{event::Event, local::LocalServer, server::Server};
    #[test]
    fn test_server_gateway() -> Result<(), Box<dyn std::error::Error>> {
        let server = Server::new(LocalServer {});

        let (sender, receiver) = server.gateway;

        // NOTE: simple for now, this will expand as other things expand
        thread::spawn(move || {
            sender.send(Event::Ping).unwrap();
        });

        assert_eq!(receiver.recv()?, Event::Ping);

        Ok(())
    }
}