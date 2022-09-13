# RUST PoC
## Proof Of Concept For Deploying Rust Apps Using Docker

This is a basic poc which exposes a LRU cache over RESTful APIs using the actix-web framework. The lru cache starts with a capacity of 2 but it can be resize using the /init endpoint.

## Build

To build the docker image, execute the following command
```sh
docker build -t poc-rust -f ./Dockerfile .
```

To run the container, execute the below command

```sh
docker run -p 9001:9001 poc-rust
```

## Endpoints and Features

- GET / exposes a static html page with list of endpoints
- POST /add can be used to add a key.
- GET /get/{key} can be used to fetch the value of {key}. 404 if the key is not present
- DELETE /clear-cache will nuke the cache
- POST /init/{size} will nuke the cache and resize it to {size}
- GET /cache-size can be used to check the size of the cache
-

### Example curl for /add

```sh
curl --location --request POST 'http://127.0.0.1:9001/add' \
--header 'Content-Type: application/json' \
--data-raw '{
    "key":"name",
    "value":"rust"
}'
```
