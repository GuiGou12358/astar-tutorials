# Astar tutorial 1
Tutorials to write Smart Contracts in Rust and Ink!

Installation: https://polkaverse.com/11143/1-technical-guide-install-cargo-contract-37738
Tutorial 4: https://polkaverse.com/11143/4-cross-contract-call-38068


## Build the contract ###
```bash
cd ./contracts/addition
cargo contract build
```
```bash
cd ./contracts/subtraction
cargo contract build
```
```bash
cd ./contracts/calculator
cargo contract build
```

## Runs the tests
```bash
cd ./contracts/addition
cargo test -- --nocapture
```
```bash
cd ./contracts/subtraction
cargo test -- --nocapture
```