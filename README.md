# Lanther

Lanther is a sample ![IPFS server](/ipfs-server/README.md), ![proxy server](/proxy-server/README.md), and ![SimpleAPI keys server](/simpleapikeys-server/README.md) built in Rust.

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

**WARNING!** This will start building docker images for all services, which will take some minutes as we are compiling multiple Rust binaries. Pre-compiled binaries should be provided at the least for the most common targets, but that work is pending.

## How to make this production ready?

Lanther is merely a sample, so there are lots of things to improve, and these are just some of them:

* Testing, of both the unit and integration variety, is lacking. 
* A CI/CD pipeline should be setup to produce pre-compiled binaries to speed up the deployment process.
* Security should be improved: 
  * All addresses as well as database passwords are completely exposed. This was done to speed up deployment since this is merely a sample. A real production deployment would store secrets in a safe manner, like using a secret manager from a cloud provider.
  * The proxy server could do more in filtering unwanted traffic. In its current implementation is very simple.
  * The SimpleAPI keys server should be secured behind an authentication layer.
* Error handling should be improved. Right now there's basic error handling without proper error messages set, or useful logging.

## What is IPFS?

IPFS is a P2P application layer protocol for file distribution, comparable to BitTorrent. Each node in an IPFS network can hold and distribute a file to other nodes in the network, as well as request data from other nodes. In order to find a node that has a file we are interested in, an IPFS network relies on a distributed hash table.

In contrast to HTTP or SAML, the P2P network composed of IPFS nodes does not rely on always-on infrastructure servers, as would be the case with a traditional client-server (centralized) architecture. 

## License

Lanther is licensed under ![MIT](LICENSE).
