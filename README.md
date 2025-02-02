# Implementing DNS over HTTPS (DoH) in Rust

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=flat&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Contributions](https://img.shields.io/badge/PRs-Welcome-brightgreen.svg)](CONTRIBUTING.md)

🚀 A high-performance, secure, and efficient DNS over HTTPS (DoH) resolver written in Rust.

## ✨ Features

- 🛡️ **Secure** - Encrypts DNS queries using HTTPS for privacy and security.
- ⚡ **High Performance** - Optimized for speed and low latency.
- 🦀 **Rust Powered** - Safe and efficient memory management.
- 🌍 **Cross-Platform** - Runs on Linux, macOS, and Windows.

## 📦 Installation

### Prerequisites
Ensure you have the following installed:
- [Rust & Cargo](https://www.rust-lang.org/tools/install)

### Clone and Build
```sh
# Clone the repository
git clone https://github.com/yourusername/dns-over-https-rust.git
cd dns-over-https-rust

# Build the project
cargo build --release
```

## 🚀 Usage

Run the DoH server:
```sh
./target/release/dns-over-https
```

By default, it will listen on `127.0.0.1:443`.

You can configure settings using a `.env` file or command-line flags.

## ⚙️ Configuration
You can customize the DoH server using a `config.toml` file:
```toml
[server]
address = "127.0.0.1"
port = 443
upstream = "https://cloudflare-dns.com/dns-query"
```

## 🛠 API Endpoints
| Method | Endpoint | Description |
|--------|---------|-------------|
| GET | `/dns-query` | Process DNS queries over HTTPS |
| POST | `/dns-query` | Alternative POST request support |

## 🧑‍💻 Contributing
We welcome contributions! Please check out the [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## 📜 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
Made with ❤️ in Rust
