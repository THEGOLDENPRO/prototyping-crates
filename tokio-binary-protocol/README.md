# tokio-binary-protocol

Run the server first in debug with:
```sh
RUST_LOG=DEBUG cargo run -p server
```

The server will bind on TCP port `8080`.

Then you can run the client:
```sh
RUST_LOG=DEBUG cargo run -p client
```

The client should connect and wait 2 seconds before sending a HELLO payload. The server should print an INFO log when it receives the payload.