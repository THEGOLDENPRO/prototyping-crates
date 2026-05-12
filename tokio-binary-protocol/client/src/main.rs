#![feature(gethostname)]

use std::{net::hostname, time::Duration};
use tokio::{net::TcpStream, time::sleep};
use futures::sink::SinkExt;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

use common::payload::{Payload, PayloadData};

// #[derive(Serialize)]
// struct WrongPayload {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Essentially wraps the TCP stream so we know where the  
    // message starts and ends in the stream of data we are receiving.
    let mut framed_write = FramedWrite::new(
        stream,
        LengthDelimitedCodec::new()
    );

    let hostname_string = hostname()
        .unwrap()
        .into_string()
        .unwrap();

    let payload = Payload {
        hostname: hostname_string,
        data: PayloadData::Hello
    };

    let payload_bytes = postcard::to_stdvec(&payload)?;

    sleep(Duration::from_secs(2)).await; // some random delay I used to debug bottlenecks on the server 

    framed_write.send(payload_bytes.to_vec().into()).await?;

    Ok(())
}