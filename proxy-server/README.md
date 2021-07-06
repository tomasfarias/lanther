# proxy-server

The lanther's app proxy-server, based on [actix's example](https://github.com/actix/examples/blob/master/basics/http-proxy/src/main.rs):

## Usage

Run the CLI with the address the proxy server will bind to and listen for requests, the address to forward on said requests to, and the address of the server that handles authentication:

```shell
proxy-server <listen addr> <listen port> <forward address> <forward port> <authentication address> <authentication port>
```
