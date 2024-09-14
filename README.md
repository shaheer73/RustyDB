# RustyDB

**RustyDB** is a distributed, high-performance, low-latency key-value store built in Rust using gRPC. It supports basic operations like `Set`, `Get`, and `Delete`, and is designed with asynchronous operations for high concurrency.

## Features

- **CRUD Operations**: Set, Get, Delete over gRPC
- **High Performance**: Asynchronous I/O with Rust's `tokio` runtime
- **Scalable Design**: Distributed architecture with simple Docker deployment
- **Low Latency**: Optimized for fast data access

## Prerequisites

Before running the project, ensure you have the following installed:

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## How to Run with Docker

### 1. Clone the repository:
```bash
git clone https://github.com/shaheer73/RustyDB.git
cd RustyDB
```

### 2. Build and run the server and client using Docker Compose:
```bash
docker-compose up --build
```

This will automatically start the server and run the client, which will perform the following operations:
- Set a key-value pair (`foo: bar`)
- Retrieve the value of `foo`
- Delete the key `foo`

### 3. Verify the Output

If everything is set up correctly, you should see output like:

```bash
server-1  | Server listening on [::]:50051
client-1  | Successfully connected to the server
client-1  | Set Response: true
client-1  | Get Response: "bar"
client-1  | Delete Response: true
client-1 exited with code 0
```

### 4. Stopping the Services
To stop the server and client, press `Ctrl + C` or run:

```bash
docker-compose down
```

## Running Manually (Without Docker)

If you prefer to run the project manually (without Docker), follow these steps:

### 1. Install Rust (if not installed):
Follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### 2. Run the Server:
In one terminal, start the server:
```bash
cargo run --bin server
```

### 3. Run the Client:
In another terminal, run the client:
```bash
cargo run --bin client
```

# Authors
Built by Noumaan & Shaheer.