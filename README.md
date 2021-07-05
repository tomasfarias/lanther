# lanther

Lanther is a sample IPFS server, proxy server, and SimpleAPI keys server built in Rust. A basic React app was also added under `app/` to interact with all the servers.

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

## License

Lanther is licensed under MIT.
