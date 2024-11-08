# Luckycoin Ord

ℹ️ This is a fork/based on [verydogelabs/wonky-ord-dogecoin](https://github.com/verydogelabs/wonky-ord-dogecoin)

## API documentation
You can find the API documentation [here](openapi.yaml).
Most convenient way to view the API documentation is to use the [Swagger Editor](https://editor.swagger.io/).
You can import the `openapi.yaml` file and view the API documentation via Import URL: `https://raw.githubusercontent.com/toregua/ord-luckycoin/main/openapi.yaml`.

## TL;DR How to run

### Preqrequisites
You will have to launch your own Luckycoin node and have it fully synced. 

You can use the following way to set up your own Luckycoin node:
1. Manually by following this documentation [install a luckycoin node manually on unix](https://github.com/luckycoin-community/luckycoin/blob/master/doc/build-unix.md).
2. Using docker (recommanded way) following this repo and documentation [run a luckycoin node inside a docker container](https://github.com/toregua/luckycoin-node)

### Ord Parameters

`--index-transactions` will store transaction data, this is currently needed for `--index-lky20` and furthermore helps
for a better performance for the API.

`--index-lky20` will store luckyscriptions data with users balance, tick list & tick holders

`--index-lunes` will store lunes data (Lune is the same concept as RUNE on BTC on DUNE on Doge)

`--nr-parallel-requests` will configure how many parallel requests while indexing are sent to your RPC Server - 16 is
recommended for default node settings.

### Env variables

To have the indexer running well you have to specify some env variables.

First you have to create a `.env`  file (you can copy paste the `.env.example` to create it)

`FIRST_INSCRIPTION_HEIGHT` for now i have set the value to 0 to be sure to handle all inscriptions. If you have the right value, you can indicate it, which will improve indexing speed.

`FIRST_LUNE_HEIGHT` for now i am not sure anyone had already deployed a LUNE on luckycoin so for now 0 also

`RPC_URL` it is here the more important part because you have to specify your node rpc url (http://user:pass@127.0.0.1:22555 for example)


## Start the lucky ord indexer / server in Docker
I recommand to use a docker image to run the ord indexer / server.

### Prerequisites Docker
1. Use ubuntu linux or a similar distribution
2. Install luckycoin node and have it fully synced (recommanded way is [run a luckycoin node inside a docker container](https://github.com/toregua/luckycoin-node))
3. Install docker and docker-compose (Ubuntu)[https://docs.docker.com/engine/install/ubuntu/]
4. Clone this repository using git clone
5. Navigate inside the cloned repository folder

### Build the Docker image
```shell
docker-compose build
```
### Start the ord in a docker container in background
```shell
docker-compose up -d
```
### Logs access
```shell
docker-compose logs -f --tail 200
```

### Stop the ord in a docker container
When stopping the ord in a container it is important to add a timeout.
If no timeout is add, the process cannot close the database properly and the next start will take ages or fail.

```shell
docker-compose stop -t 600
docker-compose down
```





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
