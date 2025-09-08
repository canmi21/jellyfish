# Lazy-MMDB

**Lazy-MMDB** is a zero-maintenance, self-updating GeoIP lookup server that provides a fast query API over a Unix Domain Socket. It leverages the MaxMind GeoIP2 databases to provide ASN, city, and country lookup capabilities for IP addresses, with automatic periodic updates to ensure data freshness.

## Features

- **Fast API**: Exposes endpoints for ASN, city, and country lookups over a Unix Domain Socket.
- **Self-Updating**: Automatically downloads and updates MaxMind GeoLite2 databases at configurable intervals.
- **Lightweight**: Built with Rust for performance and reliability.
- **Configurable**: Uses environment variables for easy customization of paths, update intervals, and database URLs.
- **Concurrent Safety**: Utilizes `parking_lot::RwLock` for thread-safe database access.

## Installation

### Prerequisites

- **Rust**: Ensure you have Rust installed (version 1.56 or later recommended). Install via [rustup](https://rustup.rs/).
- **Cargo**: Comes with Rust installation.
- **Dependencies**: The project uses external libraries like `maxminddb`, `tokio`, and `reqwest`. These are managed via `Cargo.toml`.

### Steps

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/canmi21/lazy-mmdb.git
   cd lazy-mmdb
   ```

2. **Build the Project**:
   ```bash
   cargo build --release
   ```

3. **Configure Environment** (optional):
   Copy `.env.example` to `.env` and modify as needed:
   ```bash
   cp .env.example .env
   ```
   Edit `.env` to set custom paths, log levels, or database URLs. Example:
   ```env
   LOG_LEVEL=debug
   SOCKET_PATH=/tmp/lazy-mmdb.sock
   DB_PATH=/path/to/db
   UPDATE_INTERVAL_HOURS=12
   ```

4. **Run the Server**:
   ```bash
   cargo run --release
   ```

   The server will start, download the GeoLite2 databases (if not already present), and listen on the configured Unix Domain Socket (default: `/tmp/lazy-mmdb.sock`).

## Usage

The server exposes three API endpoints over a Unix Domain Socket:

- **`/lookup/asn?ip=<IP_ADDRESS>`**: Returns ASN information for the given IP.
- **`/lookup/city?ip=<IP_ADDRESS>`**: Returns city-level geolocation data.
- **`/lookup/country?ip=<IP_ADDRESS>`**: Returns country-level geolocation data.

### Example Query

Use `curl` or a similar tool to query the API over the Unix Domain Socket:

```bash
curl --unix-socket /tmp/lazy-mmdb.sock "http://localhost/lookup/country?ip=208.67.222.222"
```

**Response** (example):
```json
{"continent":{"code":"NA","geoname_id":6255149,"names":{"de":"Nordamerika","en":"North America","es":"Norteamérica","fr":"Amérique du Nord","ja":"北アメリカ","pt-BR":"América do Norte","ru":"Северная Америка","zh-CN":"北美洲"}},"country":{"geoname_id":6252001,"iso_code":"US","names":{"de":"USA","en":"United States","es":"Estados Unidos","fr":"États Unis","ja":"アメリカ","pt-BR":"EUA","ru":"США","zh-CN":"美国"}},"registered_country":{"geoname_id":6252001,"iso_code":"US","names":{"de":"USA","en":"United States","es":"Estados Unidos","fr":"États Unis","ja":"アメリカ","pt-BR":"EUA","ru":"США","zh-CN":"美国"}}}
```

### Configuration

The server is configured via environment variables, with defaults provided in `src/config.rs`. Key settings include:

- `LOG_LEVEL`: Set to `debug`, `info`, `warn`, or `error` (default: `info`).
- `SOCKET_PATH`: Path to the Unix Domain Socket (default: `/tmp/lazy-mmdb.sock`).
- `DB_PATH`: Directory for storing MMDB files (default: `~/lazy-mmdb`).
- `UPDATE_INTERVAL_HOURS`: Database update interval in hours (default: `24`).
- `ASN_DB_URL`, `CITY_DB_URL`, `COUNTRY_DB_URL`: URLs for downloading GeoLite2 databases.

See `.env.example` for a full list of configurable options.

## Project Structure

```
lazy-mmdb/
├── src/
│   ├── api.rs          # API server logic and route handlers
│   ├── config.rs       # Configuration loading from environment variables
│   ├── db_updater.rs   # Background task for downloading and updating MMDB files
│   └── main.rs         # Application entry point
├── .env.example        # Example environment variable configuration
├── Cargo.toml          # Rust project configuration and dependencies
└── README.md           # This file
```

## Dependencies

Key dependencies include:

- `maxminddb`: For reading GeoLite2 MMDB files.
- `tokio`: For asynchronous runtime and tasks.
- `reqwest`: For downloading database files.
- `parking_lot`: For efficient concurrent state management.
- `serde` and `serde_json`: For JSON serialization.
- `lazy-sock`: For Unix Domain Socket server functionality.
- `fancy-log` and `lazy-motd`: For logging and startup messaging.

See `Cargo.toml` for a complete list.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/canmi21/lazy-mmdb).

## Acknowledgments

- [MaxMind GeoLite2](https://www.maxmind.com/) for providing free geolocation databases.
- [P3TERX](https://github.com/P3TERX/GeoLite.mmdb) for hosting the GeoLite2 database files.