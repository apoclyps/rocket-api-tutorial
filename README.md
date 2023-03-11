# Rust Rocket Tutorial

> Building a Web API with Rust using Rocket

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
 curl http://localhost:8000/heros -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='  | jq .
```


```json
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
curl http://localhost:8000/heros/1 | jq .
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
 curl -X POST http://localhost:8000/heros -H 'Content-Type: application/json' | jq .
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
 curl -X PUT http://localhost:8000/heros/1 -H 'Content-Type: application/json' | jq .
```

```json
{
  "email": "clark.kent@dailyplanet.org",
  "id": 1,
  "name": "Clark Kent"
}
```

- DELETE - Detel existing

```bash
 curl -X DELETE http://localhost:8000/heros -I
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
