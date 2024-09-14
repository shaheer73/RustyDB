# RustyDB

RustyDB is a distributed, high-performance, low-latency key-value store built in Rust using gRPC. 

## Features
- Set, Get, and Delete operations over gRPC
- High concurrency with asynchronous operations
- Distributed, scalable design

## How to Run

1. Clone the repository.
2. Start the server:
    ```bash
    cargo run --bin server
    ```
3. In a separate terminal, run the client:
    ```bash
    cargo run --bin client
    ```