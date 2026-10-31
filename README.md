# 🦀 Rust Port Scanner

A simple TCP port scanner written in Rust. I built this project to get more familiar with Rust networking and to experiment with how port scanning and basic service detection work.

The scanner checks a given IP address for open TCP ports within a specified range. When it finds an open port, it also tries to identify the service running behind it.

## Features

- **TCP port scanning** – Scan a custom range of ports on a target IP.
- **Service detection** – Attempts to identify services such as SSH, FTP, SMTP and HTTP.
- **HTTP information** – Reads the HTTP response and extracts the server header when available.
- **Connection timeout** – Uses timeouts to avoid waiting indefinitely for unreachable ports.
- **Scan duration** – Displays how long the scan took.
- **Lightweight** – No external dependencies required.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)

## Getting started

Clone the repository:

```bash
git clone https://github.com/k0dex2102/rust-port-scanner.git
cd rust-port-scanner
```

Build the project:

```bash
cargo build --release
```

## Usage

The scanner expects three arguments:

```text
<start port> <end port> <IP address>
```

Run it with Cargo:

```bash
cargo run --release -- 1 1024 127.0.0.1
```

Or execute the compiled binary directly:

```bash
./target/release/port_scanner 1 1024 127.0.0.1
```

### Example output

```text
[OPEN] 22 -> SSH (SSH-2.0-OpenSSH)
[OPEN] 80 -> HTTP (HTTP/1.1 200 OK)
Time: 1.24s
```

*Output depends on the target system and the services running on it.*

### Scan a specific range

For example, to scan ports 20 through 1000 on a local machine:

```bash
cargo run --release -- 20 1000 127.0.0.1
```

## How it works

The scanner goes through the requested port range and attempts to establish a TCP connection to each port.

If a connection succeeds, the port is reported as open. The scanner then tries to identify the service by reading its initial response or sending a basic HTTP request.

Currently, service detection covers common protocols such as SSH, FTP, SMTP and HTTP. Some ports may only be identified by their port number, while others are reported as unknown.

## Project structure

```text
rust-port-scanner/
├── src/
│   ├── main.rs
│   ├── scan.rs
│   └── service.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Development

Check the project:

```bash
cargo check
```

Run tests:

```bash
cargo test
```

Format the code:

```bash
cargo fmt
```

## Responsible use

This tool is intended for learning, testing and authorized network diagnostics.

Only scan systems and networks you own or have explicit permission to test. Unauthorized scanning may violate network policies or applicable laws.

## Contributing

Suggestions, bug reports and improvements are always welcome. Feel free to open an issue or submit a pull request.

---

Built with Rust 🦀