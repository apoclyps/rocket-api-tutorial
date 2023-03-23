# Rust Rocket Tutorial

> Building a Web API with Rust using Rocket

## Setting up the database

```bash
sudo apt-get install  libsqlite3-dev

cargo install diesel_cli --no-default-features --features sqlite

diesel setup --database-url ./database.sqlite
```

## Adding a migration

```bash
diesel migration generate create_heroes

diesel migration list --database-url=database.sqlite

diesel migration run --database-url=database.sqlite

diesel migration revert --database-url=database.sqlite
```

## Linting

You can also set up pre-commit to run the linting steps automatically during the commit phase, the pre-commit pipeline can be set up by running the following command on the project root:

```bash
pre-commit install
```

To manually run it:

```bash
pre-commit run --all
```

## Getting started

Run the service locally

```bash
cargo run
```

Make a curl request to the `/` route:

```bash
curl http://localhost:8000/ | jq .
```

Response:

```json
"Hello, world!"
```

## Endpoints

-H 'Authoorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='

- GET - List heros

```bash
 curl http://localhost:8000/heros -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' | jq .
```

```json
[
  {
    "id": 1,
    "name": "Clark Kent"
  },
  {
    "id": 2,
    "name": "Bruce Wayne"
  }
]
```

- GET - show hero

```bash
curl http://localhost:8000/heros/1  -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' | jq .
```

```json
{
  "email": "clark.kent@dailyplanet.org",
  "id": 2,
  "name": "Clark Kent"
}
```

- POST - create new

```bash
 curl -X POST http://localhost:8000/heros -H 'Content-Type: application/json'  -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -d '{"name": "John Doe", "email": "clark.kent@dailyplanet.org"}' | jq .
```

```json
{
  "email": "clark.kent@dailyplanet.org",
  "id": 3,
  "name": "John Doe"
}
```

- PUT - Update existing

```bash
 curl -X PUT http://localhost:8000/heros/1 -H 'Content-Type: application/json'  -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='  | jq .
```

```json
{
  "email": "clark.kent@dailyplanet.org",
  "id": 1,
  "name": "Clark Kent"
}
```

- DELETE - Delete existing

```bash
 curl -X DELETE http://localhost:8000/heros -I  -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
```

```bash
HTTP/1.1 404 Not Found
content-type: text/html; charset=utf-8
server: Rocket
permissions-policy: interest-cohort=()
x-content-type-options: nosniff
x-frame-options: SAMEORIGIN
content-length: 383
date: Sat, 11 Mar 2023 10:34:26 GMT

```

- Handling 404s

```bash
curl  http://localhost:8000/unsupported/api | jq .
```

```json
{
  "reason": "Not Found",
  "status": "error"
}
```
