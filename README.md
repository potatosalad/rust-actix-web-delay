# actix-web-delay

HTTP server on a given `PORT` that can simulate delayed responses.

```bash
PORT=8080 RUST_LOG=warn cargo run --release
```

Two routes are provided:

1. http://127.0.0.1:8080/random/1000 &mdash; delay the response between `0` and `1000` milliseconds
2. http://127.0.0.1:8080/static/1000 &mdash; delay the response exactly `1000` milliseconds
