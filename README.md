

---

# Luckycoin Ord

ℹ️ This is a fork based on [verydogelabs/wonky-ord-dogecoin](https://github.com/verydogelabs/wonky-ord-dogecoin)

You can see a running version here: [LUCKY-ORD.COM](https://lucky-ord.com/) - Credits to @DogepayDRC20

## API Documentation

Find the API documentation [here](openapi.yaml). You can view it conveniently in the [Swagger Editor](https://editor.swagger.io/) by importing the `openapi.yaml` file via URL: `https://raw.githubusercontent.com/toregua/ord-luckycoin/main/openapi.yaml`.

---

## Installation Guide

### 1. Prerequisites

Install dependencies:

```bash
sudo apt-get update
sudo apt-get install -y build-essential libssl-dev pkg-config curl git
```

### 2. Install Rust and Cargo

**ord-luckycoin** requires Rust to build from source. Install Rust and Cargo with:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify Rust installation:

```bash
cargo --version
```

### 3. Clone and Build ord-luckycoin

Clone the ord-luckycoin repository and build the project:

```bash
git clone https://github.com/toregua/ord-luckycoin.git
cd ord-luckycoin
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

### 4. Launch Luckycoin Node

Ensure your **Luckycoin Core** node is running and fully synced. You can install and run it manually or use Docker. Here’s a Docker guide to set up and sync a Luckycoin node: [run a luckycoin node inside a docker container](https://github.com/toregua/luckycoin-node).

After setting up **Luckycoin Core**, make sure to set `txindex=1` in `luckycoin.conf` to enable transaction indexing.

---

## TL;DR - How to Run ord-luckycoin

### ord-luckycoin Parameters

- **`--index-transactions`**: Enables storing transaction data, required for `--index-lky20` and enhances API performance.
- **`--index-lky20`**: Tracks luckyscriptions with user balances, tick list, and tick holders.
- **`--index-lunes`**: Enables tracking of lunes (analogous to BTC RUNE or Dogecoin DUNE).
- **`--nr-parallel-requests`**: Sets the number of parallel RPC requests. `16` is recommended for standard setups.

### Environment Variables

Set up a `.env` file (copy from `.env.example`) with the following:

- **`FIRST_INSCRIPTION_HEIGHT`**: Set to `0` to handle all inscriptions, or use a specific height for faster indexing.
- **`FIRST_LUNE_HEIGHT`**: Set to `0` initially; update if LUNE data is deployed on Luckycoin.
- **`RPC_URL`**: Provide your node RPC URL (e.g., `http://user:pass@127.0.0.1:22555`).

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
