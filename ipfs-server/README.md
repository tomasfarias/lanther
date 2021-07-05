# ipfs-server

A simple IPFS Server built on Rust.

## Installation

No pre-compiled binaries are yet offered, so you will have to build it yourself:

``` shell
cargo build --release
```

Once compiled, the binary can be run from anywhere.

## Usage

`ipfs-server` expects an address where to listen to requests, for example:

``` shell
./ipfs-server 127.0.0.1/8081
```
