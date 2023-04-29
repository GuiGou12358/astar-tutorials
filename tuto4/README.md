# Astar tutorial 4

Cross contract calling  
https://polkaverse.com/11143/4-cross-contract-calling-38068


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