# simpleapikeys-server

A simple server to handle API keys

## Requirements

The server relies on a MongoDB instance to store the API keys, so we should have one running.

## Installation

No pre-compiled binaries are offered yet, so you will have to build it yourself:

``` shell
cargo build --release
```

Once compiled, the binary can be run from anywhere.

Alternatively, `simpleapikeys-server` can be ran as a container, with the included Dockerfile.

## Usage

`simpleapikeys-server` expects an address where to listen to requests, as well as an address for the MongoDB instance, and a database name:

``` shell
./simpleapikeys-server 127.0.0.1:8083 --mongo-db lanther --mongo-db-address mongodb://root:supersecretpassword@localhost:27017
```

Beware! Passing credentials this way is not at all secure and is only done for demostration purposes.

Once running, the server can be interacted with `curl`, to create an API Key:

``` shell
$ curl -X POST 127.0.0.1:8083/apikeys
{"status":200,"sucess":true,"payload":{"_id":"60e4b0ef00b8983a00683f27","key":"32b1f817-9443-44fc-94aa-df893851709f","disabled":false,"created_at":"2021-07-06T19:37:19.636Z","updated_at":"2021-07-06T19:37:19.636Z"}}
```

Or get all existing API keys:

``` shell
$ curl http://127.0.0.1:8083/apikeys
{"status":200,"sucess":true,"payload":[{"_id":"60e49a4d00bdba5400754c02","key":"55ee094e-3508-4760-ab7c-33d9a4f4d460","disabled":false,"created_at":"2021-07-06T18:00:45.900Z","updated_at":"2021-07-06T18:00:45.900Z"},{"_id":"60e4b0ef00b8983a00683f27","key":"32b1f817-9443-44fc-94aa-df893851709f","disabled":false,"created_at":"2021-07-06T19:37:19.636Z","updated_at":"2021-07-06T19:37:19.636Z"}]}
```
