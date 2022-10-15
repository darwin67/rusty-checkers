# Rusty Checkers (WIP)

Simple PoC for WebAssembly building a very basic checkers game.

## Pre-requisites
* Rust

Make sure to install the `wasm` compilation target for Rust.

``` sh
make add-target
```

## Builds

``` sh
make build # debug mode
make release # optimize for release
```

The difference between the two is debug version will have debug symbols to help with debugging.

The generated wasm files will be in the following
* `target/wasm32-unknown-unknown/debug/rusty_checkers.wasm`
* `target/wasm32-unknown-unknown/release/rusty_checkers.wasm`
