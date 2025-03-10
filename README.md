# Open Protocol in Rust

**_This project is not affiliated with or endorsed by Atlas Copco._**

## What is this project?
This implements the [Open Protocol specification](https://s3.amazonaws.com/co.tulip.cdn/OpenProtocolSpecification_R280.pdf) in Rust, such that the protocol can be used in the Rust ecosystem.
This project is also a way of seeing how Rust can be used in the industrial context, as the language has still little footprint there, but has (in my opinion) high potential due to its memory safety.

## Structure
The project contains a few different components:

- [open-protocol-codec](./packages/codec): This contains the basic encoding and decoding components of the language.
- [open-protocol-codec-proc-macro](./packages/codec-proc-macro): This has some derive procedure macro implementation to help implement all the MIDs.
- [open-protocol](./packages/open-protocol): This contains all the different data structures, such as the MIDs, enums, and other types.
- [open-protocol-client](./packages/client): A library for handling the Open Protocol connection _(not implemented)_.
- [open-protocol-mqtt-adapter](./packages/mqtt-adapter): A tool to bridge Open Protocol and flat MQTT or Sparkplug B _(not implemented)_.
