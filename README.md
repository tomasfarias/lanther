# lanther

Lanther is a sample IPFS server, proxy server, and SimpleAPI keys server built in Rust.

## Requirements

Ensure the following are installed:

* [IPFS](https://ipfs.io/)
* [Docker](https://docs.docker.com/engine/install/)
* [Docker Compose](https://docs.docker.com/compose/install/)

## Setup

* Initalize an IPFS node, configure it, and run the daemon:

``` shell
ipfs init
# Any unused port would work, 6969 was arbitrarily chosen
ipfs config Addresses.Gateway /ip4/127.0.0.1/tcp/6969
ipfs daemon
```

* Ensure docker is running by, for example:

``` shell
docker ps
```

* Run everything:

``` shell
docker-compose up
```

*WARNING!* This will start building docker images for all services, which may take a few minutes as we are compiling multiple Rust binaries.

## License

Lanther is licensed under MIT.
