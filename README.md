# Placeholder

A minimal web server that displays the HTTP Host header as a placeholder page. Useful for testing reverse proxies, domain configurations, or as a simple landing page.

## Features

- Displays the incoming `Host` header as the page title and heading
- Serves static assets from `/static/`
- Graceful shutdown on SIGINT (Ctrl+C)
- Minimal Docker image using scratch base

## Usage

### Local Development

```bash
cargo run
```

The server starts on `http://0.0.0.0:8080`.

### Docker

Build and run:

```bash
docker build -t placeholder-web .
docker run -p 8080:8080 placeholder-web
```

### Release Build

```bash
cargo build --release
./target/release/placeholder-web
```

## How It Works

When you visit the server, it reads the `Host` header from your HTTP request and displays it on the page. This makes it easy to verify which domain is being used when testing proxy configurations or DNS setups.

## License

MIT
