#![cfg_attr(feature = "nightly-gethostname", feature(gethostname))]

use std::time::Duration;
use futures::{StreamExt, sink::SinkExt};
use tokio::{net::TcpStream, time::sleep};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use common::{DEFAULT_IP, DEFAULT_PORT, env_var::get_env_var, payload::{Payload, PayloadData}};

// #[derive(Serialize)]
// struct WrongPayload {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let host = get_env_var("TBP_HOST", String::from(DEFAULT_IP));
    let port = get_env_var("TBP_PORT", DEFAULT_PORT);

    let mut stream = TcpStream::connect(format!("{host}:{port}")).await?;

    // we should use the same tcp connection for both reads and writes (sending and receiving).
    let (read_half, write_half) = stream.split();

    // Essentially wraps the TCP stream so we know where the  
    // message starts and ends in the stream of data we are receiving.
    let mut framed_write = FramedWrite::new(
        write_half,
        LengthDelimitedCodec::new()
    );

    let hostname_string = {
        #[cfg(feature = "nightly-gethostname")]
        {
            use std::net::hostname;
            hostname()
                .unwrap()
                .into_string()
                .unwrap()
        }

        #[cfg(not(feature = "nightly-gethostname"))]
        {
            String::from("N/A")
        }
    };

    let payload = Payload {
        hostname: hostname_string,
        data: PayloadData::Hello
    };

    let payload_bytes = postcard::to_stdvec(&payload)?;

    sleep(Duration::from_secs(2)).await; // some random delay I used to debug bottlenecks on the server

    log::debug!("Sending HELLO payload...");

    framed_write.send(payload_bytes.into()).await?;

    // wait for the server to say hello back to us :)
    log::info!("Waiting for server to say hello back to us...");

    let mut framed_read = FramedRead::new(
        read_half,
        LengthDelimitedCodec::new()
    );

    while let Some(server_response_frame) = framed_read.next().await {
        match server_response_frame {
            Ok(payload_bytes) => {
                let payload: Payload = match postcard::from_bytes(&payload_bytes) {
                    Ok(payload) => payload,
                    Err(error) => {
                        log::debug!(
                            "Received unexpected payload from server! Error: {error}"
                        );
                        continue;
                    }
                };

                if !matches!(payload.data, PayloadData::Hello) {
                    log::debug!(
                        "Received different response from \
                        server other than HELLO! Payload: {:?}",
                        payload
                    );
                    break;
                }

                log::info!("The server said HELLO back! Payload: '{:?}'.", payload);
                break;
            },
            Err(error) => {
                log::debug!("Received unexpected response from server! Error: {error}");
            },
        }
    }

    Ok(())
}