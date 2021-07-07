# proxy-server

The lanther's app proxy-server, based on [actix's example](https://github.com/actix/examples/blob/master/basics/http-proxy/src/main.rs):

## Usage

Run the CLI with the address the proxy server will bind to and listen for requests, the address to forward on said requests to, and the address of the server that handles authentication:

```shell
proxy-server <listen addr> <listen port> <forward address> <forward port> <authentication address> <authentication port>
```

## Middlewares

The proxy-server works with a middleware to check whether a request includes the required Authorization header. This check is done against the server pointed at by the authentication parameters.

Any request that contains no Authorization header, has a API Key marked as disabled, or contains an API Key that does not exist, is rejected. Otherwise, requests are forwarded as normal.
