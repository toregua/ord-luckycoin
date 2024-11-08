# Shibes

ℹ️ This is a fork/based on [toregua/ord-luckycoin](https://github.com/toregua/ord-luckycoin)

## Key differences

‼️ DISCLAIMER: OUR CODE MAY STILL HAVE BUGS️

We included the real wonky block rewards from block 0 until block 144,999. We invite you to critically review our code in `src/epoch.rs`. We are convinced that doginals should use actual block rewards instead of a simplified version.

## API documentation
You can find the API documentation [here](openapi.yaml).
Most convenient way to view the API documentation is to use the [Swagger Editor](https://editor.swagger.io/).
You can import the `openapi.yaml` file and view the API documentation via Import URL: `https://raw.githubusercontent.com/toregua/ord-luckycoin/main/openapi.yaml`.

## TL;DR How to run

### Preqrequisites
You will have to launch your own Luckycoin node and have it fully synced. You can use the following guide to set up your own Luckycoin node:
1. Download latest version from [Luckycoin](https://github.com/luckycoin/luckycoin/releases) and install it.
   1. We have tested and launched the indexer with Luckycoin Core v1.14.8.
2. Follow the [installation instructions](https://github.com/luckycoin/luckycoin/blob/master/INSTALL.md)
   1. We started the Luckycoin Core with the following flags:
      ```shell
      luckycoind -txindex -rpcuser=foo -rpcpassword=bar -rpcport=22555 -rpcallowip=0.0.0.0/0 -rpcbind=127.0.0.1
      ```
   2. Make sure your Luckycoin node is fully synced before starting the indexer.
   3. ‼️ **IMPORTANT** Ensure to replace `foo` and `bar` with your own username and password. **IMPORTANT** ‼️
3. Start the indexer with rpc-url pointing to your Luckycoin node and the data-dir pointing to the directory where the indexer should store its data.

```shell

### Start the ord indexer / server
```shell
export RUST_LOG=info

# ensure the data directory exists
mkdir -p /mnt/ord-node/indexer-data-main

# replace YOUR_RPC_URL with the URL of your Luckycoin node like: http://foo:bar@127.0.0.1:22555

// Start Indexing
ord --rpc-url=YOUR_RPC_URL --data-dir=/mnt/ord-node/indexer-data-main --nr-parallel-requests=16 --first-inscription-height=4609723 --first-lune-height=5084000 --index-lunes --index-transactions --index-lky20 index

// Start Indexing + Server
ord --rpc-url=YOUR_RPC_URL --data-dir=/mnt/ord-node/indexer-data-main --nr-parallel-requests=16 --first-inscription-height=4609723 --first-lune-height=5084000 --index-lunes --index-transactions --index-lky20 server
```
`--index-transactions` will store transaction data, this is currently needed for `--index-lky20` and furthermore helps
for a better performance for the API.
`--nr-parallel-requests` will configure how many parallel requests while indexing are sent to your RPC Server - 16 is
recommended for default node settings.

With all settings enabled, the database will currently need around 400gb when fully indexed.

### Required env vars

## Start the ord indexer / server in Docker
You can use a docker image to run the ord indexer / server.

### Prerequisites Docker
1. Use ubuntu linux or a similar distribution
2. Install luckycoind and have it fully synced
   1See [Luckycoin installation instructions](#preqrequisites)
3. Install docker and docker-compose (Ubuntu)[https://docs.docker.com/engine/install/ubuntu/]
4. Clone this repository

### Build the Docker image
```shell
docker build -t toregua/ord-luckycoin .
```
### Start the ord in a docker container
```shell
docker compose up -d
```

### Stop the ord in a docker container
When stopping the ord in a container it is important to add a timeout.
If no timeout is add, the process cannot close the database properly and the next start will take ages or fail.

```shell
docker compose stop -t 600
docker compose down
```
