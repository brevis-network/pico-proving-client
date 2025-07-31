# Pico proving client

This repository provides a simple example of how to submit a proving task to the Pico Proving Network.

## Prerequisites

Install [Rust](https://www.rust-lang.org/tools/install) and `protoc` (required by [tonic](https://github.com/hyperium/tonic)).

## Process

Run this application with `cargo run -- --grpc-addr PICO_PROVING_NETWORK_ADDR`. After the proving is completed, you can find the generated proof in the output logs.
```
after proving, you could find the generated proof by URL:
https://pico-proofs.s3.us-west-2.amazonaws.com/task-ID/proof.bin
```
