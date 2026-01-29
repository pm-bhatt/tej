# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-29

### Added

- **tej-core**: Core speed test measurement library
  - Download speed measurement with adaptive sizing (100KB-25MB based on connection speed)
  - Upload speed measurement with 6 parallel connections
  - Latency measurement with 20 samples (3 warmup discarded)
  - Jitter calculation using RFC 3550 algorithm
  - Packet loss detection via HTTP timeout analysis
  - Real-time progress callbacks for UI integration
  - Incompressible random data generation for honest measurements

- **tej-cli**: Command-line interface
  - Beautiful terminal output with progress bars
  - JSON output format for scripting (`--format json`)
  - Configurable parallel connections (`-c 1-32`)
  - Skip download/upload options (`--no-download`, `--no-upload`)

- **tej-gui**: Cross-platform desktop application (Tauri 2.0 + Svelte)
  - SVG speed gauge with animated needle
  - Real-time speed display during tests
  - Test history persistence
  - Dark theme UI

### Technical Highlights

- Uses Cloudflare CDN (300+ global edge locations) as test infrastructure
- Incompressible random data prevents ISP traffic optimization cheating
- Zero-copy upload using `bytes::Bytes` for minimal memory footprint
- Proper atomic ordering (Release/Acquire) for accurate multi-threaded measurements
- Input validation to prevent resource exhaustion attacks

### Platforms

- macOS (Apple Silicon & Intel)
- Windows
- Linux
- iOS (coming soon)
- Android (coming soon)

[0.1.0]: https://github.com/pm-bhatt/tej/releases/tag/v0.1.0
