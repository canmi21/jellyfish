# Jellyfish

A lightweight static resource server built with [Axum](https://crates.io/crates/axum).

## Features

- **Static File Serving**: Efficiently serves static files from a specified directory.
- **SPA Mode**: Optional Single Page Application (SPA) mode to fallback to `index.html` for non-existent routes.
- **API Endpoints**: Supports `?info` for file metadata and `?list` for directory listings.
- **Secure Path Handling**: Prevents path traversal attacks with strict path cleaning.
- **Custom Error Pages**: Serves a custom `404.html` if available, with a fallback default.
- **Configurable**: Environment variables for log level, port, public directory, and SPA mode.
- **Docker Support**: Ready-to-use Docker configuration for easy deployment.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable, latest recommended)
- [Docker](https://www.docker.com/get-started) (optional, for containerized deployment)

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/canmi21/jellyfish.git
   cd jellyfish
   ```

2. Build and run:
   ```bash
   cargo build --release
   cargo run --release
   ```

3. Configure environment variables (optional, see `.env.example`):
   ```bash
   cp .env.example .env
   nano .env
   ```

### Using Docker

1. Build the Docker image:
   ```bash
   docker build -t canmi/jellyfish .
   ```

2. Run with Docker Compose:
   ```bash
   docker-compose up -d
   ```

## Configuration

The server is configured via environment variables. See `.env.example` for defaults:

- `LOG_LEVEL`: Logging level (`debug`, `info`, `warn`, `error`). Default: `info`.
- `BIND_PORT`: Port to bind the server. Default: `33433`.
- `PUBLIC_DIR`: Directory to serve static files from (supports `~` for home directory). Default: `~/jellyfish/public`.
- `INDEX_ROUTER_MODE`: SPA mode (`true`/`false`). If `true`, non-existent routes fallback to `index.html`. Default: `false`.

Example `.env`:
```bash
LOG_LEVEL=info
BIND_PORT=33433
PUBLIC_DIR=~/jellyfish/public
INDEX_ROUTER_MODE=false
```

## Usage

- **Access the server**: Open `http://localhost:33433` in your browser.
- **File Info API**: Append `?info` to a file path (e.g., `http://localhost:33433/file.txt?info`) to get metadata like size, modification time, and XXH64 hash.
- **Directory Listing API**: Append `?list` to a directory path (e.g., `http://localhost:33433/folder/?list`) to list directory contents.
- **SPA Mode**: Enable `INDEX_ROUTER_MODE=true` to serve `index.html` for all non-existent routes, ideal for single-page applications.

## Project Structure

```
jellyfish/
├── index/              # Default index.html and 404.html templates
├── public/             # Static files to serve
├── src/                # Source code
│   ├── config.rs       # Configuration loading and setup
│   ├── handler.rs      # Request handling logic
│   ├── main.rs         # Application entry point
│   ├── response.rs     # Response formatting utilities
│   ├── server.rs       # Axum router setup
│   └── shutdown.rs     # Graceful shutdown handling
├── .env.example        # Example environment configuration
├── Cargo.toml          # Rust dependencies and metadata
├── docker-compose.yml  # Docker Compose configuration
├── Dockerfile          # Docker build instructions
└── README.md           # This file
```

## Development

To contribute or modify the project:

1. Install Rust and dependencies.
2. Make changes to the source code in `src/`.
3. Test locally:
   ```bash
   cargo run
   ```
4. Submit a pull request to [https://github.com/canmi21/jellyfish](https://github.com/canmi21/jellyfish).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.