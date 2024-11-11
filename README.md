
---

# Luckycoin Ord

ℹ️ This project is a fork based on [verydogelabs/wonky-ord-dogecoin](https://github.com/verydogelabs/wonky-ord-dogecoin).

You can see a running version here: [LUCKY-ORD.COM](https://lucky-ord.com/) - Credits to @DogepayDRC20.

## API Documentation

Find the API documentation [here](openapi.yaml). You can view it conveniently in the [Swagger Editor](https://editor.swagger.io/) by importing the `openapi.yaml` file via URL: `https://raw.githubusercontent.com/toregua/ord-luckycoin/main/openapi.yaml`.

---

## Installation Guide

### Prerequisites

Install required dependencies:

```bash
sudo apt-get update
sudo apt-get install -y build-essential libssl-dev pkg-config curl git
```

### Install Rust and Cargo

**ord-luckycoin** requires Rust to build from source. Install Rust and Cargo with:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify the Rust installation:

```bash
cargo --version
```

### Clone and Build ord-luckycoin

Clone the **ord-luckycoin** repository and build the project:

```bash
git clone https://github.com/toregua/ord-luckycoin.git
cd ord-luckycoin
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

### Start Luckycoin Node

Ensure your **Luckycoin Core** node is running and fully synced. You can install and run it manually or use Docker. Here’s a Docker guide to set up and sync a Luckycoin node: [run a luckycoin node inside a docker container](https://github.com/toregua/luckycoin-node).

Make sure `txindex=1` is set in `luckycoin.conf` to enable transaction indexing, as **ord-luckycoin** requires this.

---

## Running ord-luckycoin Indexer and Server

### Command Line Options

- **`--index-transactions`**: Enables storing transaction data, required for `--index-lky20` and enhances API performance.
- **`--index-lky20`**: Tracks luckyscriptions with user balances, tick list, and tick holders.
- **`--index-lunes`**: Enables tracking of lunes (analogous to BTC RUNE or Dogecoin DUNE).
- **`--nr-parallel-requests`**: Sets the number of parallel RPC requests. `16` is recommended for standard setups.

### Environment Variables

Set up a `.env` file (copy from `.env.example`) with the following:

- **`FIRST_INSCRIPTION_HEIGHT`**: Set to `0` to handle all inscriptions, or use a specific height for faster indexing.
- **`FIRST_LUNE_HEIGHT`**: Set to `0` initially; update if LUNE data is deployed on Luckycoin.
- **`RPC_URL`**: Provide your node RPC URL (e.g., `http://user:pass@127.0.0.1:9918`).

---

### Start the ord-luckycoin Indexer and Server

To start the **ord-luckycoin** indexer and server in the foreground:

```bash
./target/release/ord \
    --first-inscription-height=0 \
    --rpc-url=http://127.0.0.1:9918/ \
    --data-dir=/root/.local/share/ord \
    server --http-port=8080
```

This command:
- Starts the indexer from block `0` and connects to the Luckycoin node at `127.0.0.1:9918`.
- Specifies the data directory at `/root/.local/share/ord`.
- Runs the server on HTTP port `8080`.

This will keep the process running in the foreground so that you can monitor it directly.

---

## Accessing Logs (Optional)

If you prefer to capture logs, you can redirect the output to a log file:

```bash
./target/release/ord \
    --first-inscription-height=0 \
    --rpc-url=http://127.0.0.1:9918/ \
    --data-dir=/root/.local/share/ord \
    server --http-port=8080 > ~/ord-luckycoin.log 2>&1
```

To view the log output in real-time, open a new terminal and use:

```bash
tail -f ~/ord-luckycoin.log
```

---

## Running ord-luckycoin in Docker

Using Docker to run the ord-luckycoin indexer/server is recommended.

### Docker Setup

1. **System Requirements**: Ubuntu or a similar Linux distribution.
2. **Luckycoin Node**: Install and sync a Luckycoin node (recommended via [Docker guide](https://github.com/toregua/luckycoin-node)).
3. **Install Docker and Docker-Compose**: [Docker installation guide](https://docs.docker.com/engine/install/ubuntu/).
4. **Clone Repository**: Clone this repository and navigate to the cloned directory.

### Build and Start Docker Image

To build and run the **ord-luckycoin** Docker container:

```shell
# Build Docker image
docker-compose build

# Start ord-luckycoin in background
docker-compose up -d

# View logs
docker-compose logs -f --tail 200
```

### Stop Docker Container

When stopping the container, add a timeout to avoid database issues.

```shell
docker-compose stop -t 600
docker-compose down
```

---

