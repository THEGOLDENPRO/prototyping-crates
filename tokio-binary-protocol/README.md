# tokio-binary-protocol

Run the server first in debug with:
```sh
RUST_LOG=DEBUG cargo run -p server
```

> The server will bind on `localhost` TCP port `7185` by default. You can change this on both the client and server with environment variables `TBP_HOST` and `TBP_PORT`.

Then you can run the client:
```sh
RUST_LOG=DEBUG cargo run -p client
```

The client should connect and wait 2 seconds before sending a HELLO payload. When the server receives the HELLO it should print an INFO log and then respond back to the client also with a hello. The client will wait for this HELLO and terminate when it receives it.