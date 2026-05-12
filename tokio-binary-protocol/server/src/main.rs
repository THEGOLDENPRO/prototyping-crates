use std::process::ExitCode;

use futures::StreamExt;
use tokio::{net::TcpListener};
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

use common::payload::Payload;

use crate::error::{Error, Result};

mod error;

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();

    if let Err(error) = start_server().await {
        log::error!("{}", error);

        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

pub async fn start_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await
        .map_err(|error| Error::TCPListenerBindFailure { error: error.to_string() })?;

    log::info!("Server is listening...");

    loop {
        let (tcp_stream, address) = listener.accept().await
            .map_err(|error| Error::TCPListenerAcceptConnectionsFailure { error: error.to_string() })?;

        tokio::spawn(
            async move {
                log::debug!("Accepting new TCP connection from '{address}'...");

                // Of course on the receiving end we also got to 
                // wrap the TCP stream but in this case with 
                // "FramedRead" instead of "FramedWrite".
                let mut framed_read = FramedRead::new(
                    tcp_stream,
                    LengthDelimitedCodec::new()
                );

                while let Some(next_frame) = framed_read.next().await {
                    match next_frame {
                        Ok(payload_bytes) => {
                            let payload: Payload = match postcard::from_bytes(&payload_bytes) {
                                Ok(payload) => payload,
                                Err(error) => {
                                    log::debug!(
                                        "Received unexpected payload from '{address}'! Error: {error}"
                                    );
                                    continue;
                                }
                            };

                            log::info!("Received '{:?}' from '{}'.", payload, address);
                        },
                        Err(error) => {
                            log::debug!("Received unexpected data from '{address}'! Error: {error}");
                        },
                    }
                }
            }
        );


    }
}