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

## Usage

Lanther lacks a frontend application, but can be interacted with `curl` or any other similar tool.

An API key may be created by making a `POST` request to the `simpleapikeys-server`:

``` shell
curl -X POST 127.0.0.1:8083/apikeys
{"status":200,"success":true,"payload":{"_id":"60e7120900cb9f12006e3176","key":"c814216a-abd0-4301-a732-abc6f8ed5e5b","disabled":false,"created_at":"2021-07-08T14:56:09.072Z","updated_at":"2021-07-08T14:56:09.072Z"}}
```

Send a `GET` request to the `proxy-server` and get promptly rejected for not including an API key:

``` shell
curl -X POST 127.0.0.1:8081/ -v
*   Trying 127.0.0.1:8081...
* Connected to 127.0.0.1 (127.0.0.1) port 8081 (#0)
> POST / HTTP/1.1
> Host: 127.0.0.1:8081
> User-Agent: curl/7.77.0
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 401 Unauthorized
< content-length: 18
< content-type: text/plain; charset=utf-8
< date: Thu, 08 Jul 2021 14:57:16 GMT
<
* Connection #0 to host 127.0.0.1 left intact
APIKey is required
```

Include the API key created at the beginning, and we can now send a payload to the `ipfs-server` to upload:

``` shell
curl -X POST 127.0.0.1:8081/ -H "Authorization: c814216a-abd0-4301-a732-abc6f8ed5e5b" -d "Hello, World\!\\n" -v
Note: Unnecessary use of -X or --request, POST is already inferred.
*   Trying 127.0.0.1:8081...
* Connected to 127.0.0.1 (127.0.0.1) port 8081 (#0)
> POST / HTTP/1.1
> Host: 127.0.0.1:8081
> User-Agent: curl/7.77.0
> Accept: */*
> Authorization: c814216a-abd0-4301-a732-abc6f8ed5e5b
> Content-Length: 15
> Content-Type: application/x-www-form-urlencoded
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 125
< date: Thu, 08 Jul 2021 15:10:44 GMT
< content-type: application/json
<
* Connection #0 to host 127.0.0.1 left intact
{"hash":"QmczdVVHRGBvWMQxAYQuprz8cSsGyodhCYg55RdUmD1LHB","name":"QmczdVVHRGBvWMQxAYQuprz8cSsGyodhCYg55RdUmD1LHB","size":"23"}
```

A quick `ipfs cat` reveals that our content was uploaded correctly:

``` shell
ipfs cat QmczdVVHRGBvWMQxAYQuprz8cSsGyodhCYg55RdUmD1LHB
Hello, World!\n
```

If we were to disable the API key, our requests to the proxy server would stop working:

``` shell
curl -X PUT 127.0.0.1:8083/apikeys -H "Content-Type: application/json" -d '{"key":"c814216a-abd0-4301-a732-abc6f8ed5e5b","disabled":true}'
{"status":200,"success":true,"payload":{"_id":"60e7120900cb9f12006e3176","key":"c814216a-abd0-4301-a732-abc6f8ed5e5b","disabled":true,"created_at":"2021-07-08T14:56:09.072Z","updated_at":"2021-07-08T15:14:21.716Z"}}
# Our API key is now disabled, requests using it will no longer succeed
curl 127.0.0.1:8081/ -H "Authorization: c814216a-abd0-4301-a732-abc6f8ed5e5b" -d "Not going anywhere\\n" -v
*   Trying 127.0.0.1:8081...
* Connected to 127.0.0.1 (127.0.0.1) port 8081 (#0)
> POST / HTTP/1.1
> Host: 127.0.0.1:8081
> User-Agent: curl/7.77.0
> Accept: */*
> Authorization: c814216a-abd0-4301-a732-abc6f8ed5e5b
> Content-Length: 20
> Content-Type: application/x-www-form-urlencoded
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 401 Unauthorized
< content-length: 18
< content-type: text/plain; charset=utf-8
< date: Thu, 08 Jul 2021 15:15:29 GMT
<
* Connection #0 to host 127.0.0.1 left intact
APIKey is disabled
```

The `/requests` path allows us to list requests:

``` shell
curl 127.0.0.1:8081/requests | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  1168  100  1168    0     0  1077k      0 --:--:-- --:--:-- --:--:-- 1140k
{
  "status": 200,
  "sucess": true,
  "payload": [
    {
      "_id": "60e64303003ab3ab00bb1afb",
      "method": "GET",
      "path": "/",
      "authorization": "e6eb636b-0109-4a27-a211-5b96472c0642",
      "created_at": "2021-07-08T00:12:51.320Z"
    },
    {
      "_id": "60e71574007e6e9700a52983",
      "method": "POST",
      "path": "/",
      "authorization": "c814216a-abd0-4301-a732-abc6f8ed5e5b",
      "created_at": "2021-07-08T15:10:44.462Z"
    }
  ]
}
```

Finally, we may also create a new API key, list all keys, use that key to send more data, and list all requests done by that specific key:

``` shell
curl -X POST 127.0.0.1:8083/apikeys | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   216  100   216    0     0  43002      0 --:--:-- --:--:-- --:--:-- 54000
{
  "status": 200,
  "success": true,
  "payload": {
    "_id": "60e717500030d9c600e4219c",
    "key": "ea3c042b-204a-4272-adb3-3e32695ffd6b",
    "disabled": false,
    "created_at": "2021-07-08T15:18:40.351Z",
    "updated_at": "2021-07-08T15:18:40.351Z"
  }
}
curl 127.0.0.1:8083/apikeys | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   570  100   570    0     0   224k      0 --:--:-- --:--:-- --:--:--  278k
{
  "status": 200,
  "sucess": true,
  "payload": [
    {
      "_id": "60e4c275006c18e400d63f26",
      "key": "e6eb636b-0109-4a27-a211-5b96472c0642",
      "disabled": false,
      "created_at": "2021-07-06T20:52:05.400Z",
      "updated_at": "2021-07-07T22:56:24.903Z"
    },
    {
      "_id": "60e7120900cb9f12006e3176",
      "key": "c814216a-abd0-4301-a732-abc6f8ed5e5b",
      "disabled": true,
      "created_at": "2021-07-08T14:56:09.072Z",
      "updated_at": "2021-07-08T15:14:21.716Z"
    },
    {
      "_id": "60e717500030d9c600e4219c",
      "key": "ea3c042b-204a-4272-adb3-3e32695ffd6b",
      "disabled": false,
      "created_at": "2021-07-08T15:18:40.351Z",
      "updated_at": "2021-07-08T15:18:40.351Z"
    }
  ]
}
curl 127.0.0.1:8081/ -H "Authorization: ea3c042b-204a-4272-adb3-3e32695ffd6b" -d "More data\\n"
{"hash":"QmfLnX6eX1g1qffpBL5MBStKg1ga6hVQYi36wLGdjqjSmV","name":"QmfLnX6eX1g1qffpBL5MBStKg1ga6hVQYi36wLGdjqjSmV","size":"19"}
curl 127.0.0.1:8081/requests/ea3c042b-204a-4272-adb3-3e32695ffd6b | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   198  100   198    0     0  61300      0 --:--:-- --:--:-- --:--:-- 66000
{
  "status": 200,
  "success": true,
  "payload": [
    {
      "_id": "60e71771007e6e9700a52984",
      "method": "POST",
      "path": "/",
      "authorization": "ea3c042b-204a-4272-adb3-3e32695ffd6b",
      "created_at": "2021-07-08T15:19:13.496Z"
    }
  ]
}
```

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

In contrast to HTTP or SAML, theP2P network composed of IPFS nodes does not rely on always-on infrastructure servers, as would be the case with a traditional client-server (centralized) architecture. 

## License

Lanther is licensed under ![MIT](LICENSE).
