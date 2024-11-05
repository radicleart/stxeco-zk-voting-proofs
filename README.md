# zk-server

zk-server is a backend to the zk-vote community voting application. Its role is to
provide users with anonymous voting rights.

## API

Server maintains a websocket connection with the client and can be run as a server or
downloaded to the client computer and run to avoid uploading private inputs.

## Building

To create a production version of your app:

```bash
cargo run

Listening on: 127.0.0.1:9001
```

## Technology

zk-vote is built on zk-stark technology which provides the following benefits;

- **Transparency**: zk-STARKs are transparent and do not require a trusted setup, enhancing security and reducing the potential for “trapdoors.”
- **Scalability**: zk-STARKs are highly scalable, with proof generation that can handle larger datasets efficiently.
- **Quantum Resistance**: zk-STARKs are resistant to quantum attacks due to their reliance on hash functions rather than elliptic curve cryptography.
- **Auditability and Transparency**: zk-STARKs have transparent proof generation, making them highly auditable and appealing for public blockchain use.