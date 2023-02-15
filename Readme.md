
# Basic P2P implementation with Geth Node

This project implements basic p2p network handshake of Geth Node using rust.



## Pre-requisite

1. Install a local geth node. Follow the following steps to get it running locally on your machine (WIN, LINUX or MAC)
- Download the core-geth binary from :https://github.com/etclabscore/core-geth/releases
- Extract it and install
- To run the local geth node execute the following command: `geth --http` . This will run a local node with the default node id = 1.
2. Rust setup in your system and this code base cloned in your system.


## Code Configuration

1. Note the geth node endpoint. If you are running it locally on the system where you intend to execute the code use:
   NODE_ENPOINT= "http://127.0.0.1:8545"
   and
   `NODE_ID= 1`. Please note that if you are adding desired node ID to the geth node, please change the configuration in `.env` file accordingly.


## Execution

In order to execute the code, please use the following command:

`cargo run` in the base directory (this will download the dependencies and execute the code).


## OUTPUT
Execution log of geth node:

<img src="https://ik.imagekit.io/m5gtndqap/Screenshot_2023-02-15_at_9.20.25_PM.png?ik-sdk-version=javascript-1.4.3&updatedAt=1676477160046"/>

Execution log of gethp2p:

<img src="https://ik.imagekit.io/m5gtndqap/Screenshot_2023-02-15_at_9.20.11_PM.png?ik-sdk-version=javascript-1.4.3&updatedAt=1676477159350"/>

