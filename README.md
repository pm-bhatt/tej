<p align="center">
  <h1 align="center">Tej</h1>
  <p align="center"><strong>Honest Internet Speed Test</strong></p>
  <p align="center">
    <em>tej (तेज़) — Hindi for "fast, swift"</em>
  </p>
</p>

<p align="center">
  <a href="https://github.com/pm-bhatt/tej/actions"><img src="https://img.shields.io/github/actions/workflow/status/pm-bhatt/tej/ci.yml?style=flat-square" alt="Build Status"></a>
  <a href="https://github.com/pm-bhatt/tej/releases"><img src="https://img.shields.io/github/v/release/pm-bhatt/tej?style=flat-square" alt="Release"></a>
  <a href="https://github.com/pm-bhatt/tej/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License"></a>
</p>

---

## Why Tej?

Most speed test tools use **compressible data** that ISPs can optimize in transit, giving you inflated results. Tej uses **incompressible random data** to show your *real* internet speed.

| Feature | Tej | Ookla | Fast.com |
|---------|-----|-------|----------|
| Incompressible data | Yes | No | No |
| Open source | Yes | No | No |
| Cross-platform (Desktop + Mobile) | Yes | Yes | Web only |
| Lightweight (<10MB binary) | Yes | No (~200MB) | N/A |
| No telemetry | Yes | No | No |

## Installation

### Homebrew (macOS/Linux)

```bash
brew tap pm-bhatt/tej
brew install tej
```

### Cargo

```bash
cargo install tej-cli
```

### Download Binary

Download pre-built binaries from [Releases](https://github.com/pm-bhatt/tej/releases).

### GUI App

Download from [Releases](https://github.com/pm-bhatt/tej/releases) for:
- macOS (Apple Silicon & Intel)
- Windows
- Linux (AppImage, deb)
- iOS (coming soon)
- Android (coming soon)

### Build from Source

```bash
git clone https://github.com/pm-bhatt/tej.git
cd tej

# CLI only
cargo build --release -p tej-cli

# GUI (requires Node.js)
cd apps/tej-gui
npm install
cargo tauri build
```

## Usage

### CLI

```bash
# Basic speed test
tej

# JSON output (for scripts)
tej --format json

# Customize connections
tej -c 8

# Skip upload test
tej --no-upload

# All options
tej --help
```

### Example Output

```
Tej - Honest Speed Test
Testing with 6 parallel connections...

┌─────────────────┬────────────────┐
│ Metric          │ Value          │
├─────────────────┼────────────────┤
│ Server          │ SFO            │
│ Latency (avg)   │ 12.3 ms        │
│ Latency (min/max)│ 10.1 / 18.2 ms │
│ Jitter          │ 1.8 ms         │
│ Download        │ 245.67 Mbps    │
│ Upload          │ 98.34 Mbps     │
│ Packet Loss     │ 0.0%           │
└─────────────────┴────────────────┘
```

## How It Works

1. **Latency Measurement**: 20 HTTP requests to Cloudflare's edge, discarding first 3 as warmup
2. **Jitter Calculation**: Mean absolute difference between consecutive samples (RFC 3550)
3. **Download Test**: 6 parallel HTTP GET streams with incompressible random data
4. **Upload Test**: 6 parallel HTTP POST with locally-generated random bytes
5. **Packet Loss**: HTTP timeout-based approximation over 20 requests

### Why Cloudflare?

- 300+ edge locations worldwide
- Consistent, reliable infrastructure
- Representative of real-world CDN performance
- `cf-ray` header tells you exactly which datacenter served you

## Architecture

```
tej/
├── crates/
│   ├── tej-core/     # Core measurement library
│   └── tej-cli/      # Terminal CLI
└── apps/
    └── tej-gui/      # Tauri 2.0 desktop/mobile app
        ├── src-tauri/    # Rust backend
        └── src/          # Svelte frontend
```

## Metrics Explained

| Metric | What It Measures |
|--------|------------------|
| **Download** | Throughput receiving data (Mbps) |
| **Upload** | Throughput sending data (Mbps) |
| **Latency** | Round-trip time to server (ms) |
| **Jitter** | Variation in latency (ms) - lower is better |
| **Packet Loss** | Percentage of failed requests |

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Run tests
cargo test --workspace

# Run clippy
cargo clippy --workspace -- -D warnings

# Format code
cargo fmt --all
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [Cloudflare](https://cloudflare.com) for providing the speed test infrastructure
- Built with [Rust](https://rust-lang.org), [Tauri](https://tauri.app), and [Svelte](https://svelte.dev)
