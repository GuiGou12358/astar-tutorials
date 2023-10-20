# Astar tutorial 10

Smart contract verification
https://polkaverse.com/11143/verify-a-ink-smart-contract-with-patron-39541


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

## verify the contracts
```bash
patron build --root contracts/addition/
```
```bash
patron build --root contracts/subtraction/
```
```bash
patron build --root contracts/calculator/
```