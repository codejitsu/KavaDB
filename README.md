# KavaDB
Simple distributed KV store with consistent hashing and gossip protocol written in Rust for maximum efficiency.

# Supported operations

## Consistent Hashing

The system uses consistent hashing to distribute keys across multiple nodes in the cluster. Each node is assigned a position on the hash ring, and keys are mapped to nodes based on their hash values. This allows for efficient key distribution and minimizes data movement when nodes are added or removed.

- PUT key value
- READ key
- DELETE key
- BATCHPUT key1 value1 key2 value2 ...

## No consistent hashing (TODO)

This command does not use consistent hashing and stores all keys in the local node only.

- RANGE start_key end_key

# Features
- In-memory storage with optional persistence (TODO)
- Gossip protocol for node discovery and cluster membership
- Simple command-line interface for interacting with the KV store
- Basic error handling and logging
- Unit tests for core functionality
- Basic consistent hashing for key distribution
- Virtual nodes for better distribution

# Prerequisites
- Rust 1.70+
- Cargo
- Git

# Getting Started
1. Clone the repository:
   ```bash
   git clone https://github.com/codejitsu/KavaDB.git
   ```
2. Install Rust and Cargo if you haven't already. You can follow the instructions at [rustup.rs](https://rustup.rs/).

3. Change into the project directory:
   ```bash
   cd KavaDB/node
   ```
4. Build the project:
   ```bash
   cargo build
   ```
5. Run the project:
   ```bash
   cargo run
   ```

# Example Usage with Multiple Nodes
To simulate a distributed environment, you can run multiple instances of the application with different configuration files.

Open multiple terminal windows and run the following commands:
```bash
# Terminal 1
cargo run -- kava.conf

# Terminal 2
cargo run -- kava2.conf

# Terminal 3
cargo run -- kava3.conf
```

Now you can interact with any of the nodes using the command-line interface. For example, in Terminal 1, you can run:
```bash
echo "PUT nickname codejitsu" | nc localhost 3001
```

Now you can read the value from any node:
```bash
echo "READ nickname" | nc localhost 3002
```

# Configuration
The application can be configured using the `kava.conf` file located in the project directory.

There are three config files provided for testing:
- `kava1.conf`
- `kava2.conf`
- `kava3.conf`

Each config file represents a different node in the cluster. You can run multiple instances of the application with different config files to simulate a distributed environment.

# Running Tests
To run the tests, use the following command:
```bash
cargo test
```