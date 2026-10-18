🦀 Rust Port Scanner

A lightweight port scanner written in Rust, designed to explore TCP port scanning and network diagnostics.

✨ Features
⚡ Performance — Built with Rust for efficient execution.
🌐 Network Scanning — Designed to check TCP ports on a target host.
🎯 Port Targeting — Intended for scanning specific ports or port ranges.
🛡️ Memory Safety — Benefits from Rust's memory-safe design.
🔧 Open Source — Easy to explore, modify, and extend.
Note: The exact feature set depends on the current implementation.
📋 Requirements
Rust
Cargo (included with Rust)
🚀 Installation

Clone the repository:

git clone https://github.com/k0dex2102/rust-port-scanner.git
cd rust-port-scanner


Build the project:

cargo build --release

💻 Usage

Run the project using Cargo:

cargo run --release


For available commands and arguments, check the program's help output or source code.

Example

A typical port scanner can be used to check a target host for reachable TCP ports.

Example target: 127.0.0.1 (localhost).

Only scan systems you own or have explicit permission to test.

🛠️ Development

Run the project in development mode:

cargo run


Check the code for common issues:

cargo check


Run the test suite:

cargo test


Format the source code:

cargo fmt

📁 Project Structure
rust-port-scanner/
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md


The structure above is an example and may differ from the actual repository.

🔒 Responsible Use

This project is intended for educational purposes and authorized network diagnostics.

Only scan systems you own or have explicit permission to test.
Respect network policies and applicable laws.
Avoid scanning networks without authorization.

The author is not responsible for misuse of this software.

🤝 Contributing

Contributions, bug reports, and feature suggestions are welcome.

Fork the repository.
Create a feature branch.
Make your changes.
Submit a pull request.
📄 License

See the LICENSE file for license information.

Built with 🦀 Rust

Repository