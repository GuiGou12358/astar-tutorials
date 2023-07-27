# Astar tutorial 7

Contract Testing
https://polkaverse.com/11143/7-contract-testing-38955


## Build the contract ###
```bash
cd ./incrementer
cargo +nightly contract build
```

## Unit adn Integration tests  ###
```bash
cargo +nightly test
```


## e2e tests  ###
Before you can run the test, you have to install a Substrate node with pallet-contracts. By default, e2e tests require that you install substrate-contracts-node.

To install the latest version:
```bash
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git
```
You can also use Swanky node. The easiest method of installation is by downloading and executing a precompiled binary from the Release Page.
And you need to change CONTRACTS_NODE environment variable:
```bash
export CONTRACTS_NODE="YOUR_CONTRACTS_NODE_PATH"
```
You do not need to run it in the background since the node is started for each test independently.

And finally execute the following command to start e2e test execution.

```bash
cargo test --features e2e-tests
```