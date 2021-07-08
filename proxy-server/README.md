# proxy-server

The lanther's app proxy-server, based on [actix's example](https://github.com/actix/examples/blob/master/basics/http-proxy/src/main.rs):

## Usage

Run the CLI with the address the proxy server will bind to and listen for requests, the address to forward on said requests to, the address of the server that handles authentication, and the address to the MongoDB instance used to store requests:

```shell
proxy-server <listen addr> <listen port> <forward address> <forward port> <authentication address> <authentication port> --mongo-db <db name> --mongo-db-address <mongo db address>
```

## Authorization middleware

The proxy-server works with a middleware to check whether a request includes the required Authorization header. This check is done against the server pointed at by the authentication parameters.

Any request that contains no Authorization header, has a API Key marked as disabled, or contains an API Key that does not exist, is rejected. Otherwise, requests are forwarded as normal.

## Request logging

Every time a request is received, it is logged to MongoDB. These requests can be fetched by a `GET` request to `/requests`:

``` shell
# Retrieves all requests
curl http://127.0.0.1:8081/requests
# Retrieves all requests matching key e6eb636b-0109-4a27-a211-5b96472c0642
curl http://127.0.0.1:8081/requests/e6eb636b-0109-4a27-a211-5b96472c0642
```
