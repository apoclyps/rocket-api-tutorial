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
