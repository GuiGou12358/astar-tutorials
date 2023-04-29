# Astar tutorial 3

Polymorphism with 'ink::trait_definition'  
https://polkaverse.com/11143/3-polymorphism-with-ink-trait-definition-38055

## Build the contract ###
```bash
cd ./contracts/addition
cargo contract build
```
```bash
cd ./contracts/subtraction
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