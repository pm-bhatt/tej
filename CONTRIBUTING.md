# Contributing to Tej

Thank you for your interest in contributing to Tej! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/pm-bhatt/tej/issues)
2. If not, create a new issue with:
   - Clear, descriptive title
   - Steps to reproduce
   - Expected vs actual behavior
   - Your environment (OS, Rust version, etc.)

### Suggesting Features

1. Check existing issues and discussions first
2. Open a new issue with the `enhancement` label
3. Describe the feature and its use case

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Ensure tests pass: `cargo test --workspace`
5. Ensure clippy passes: `cargo clippy --workspace -- -D warnings`
6. Format code: `cargo fmt --all`
7. Commit with clear message: `git commit -m "feat: add your feature"`
8. Push and open a PR

## Development Setup

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs))
- Node.js 18+ (for GUI development)
- Platform-specific dependencies for Tauri (see [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

### Building

```bash
# Clone the repo
git clone https://github.com/pm-bhatt/tej.git
cd tej

# Build all crates
cargo build --workspace

# Run CLI
cargo run -p tej-cli

# Run tests
cargo test --workspace

# Run GUI in dev mode
cd apps/tej-gui
npm install
cargo tauri dev
```

## Project Structure

```
tej/
├── Cargo.toml              # Workspace configuration
├── crates/
│   ├── tej-core/           # Core measurement library
│   │   └── src/
│   │       ├── lib.rs      # Public API
│   │       ├── config.rs   # Test configuration
│   │       ├── download.rs # Download measurement
│   │       ├── upload.rs   # Upload measurement
│   │       ├── latency.rs  # Latency measurement
│   │       ├── jitter.rs   # Jitter calculation
│   │       └── ...
│   └── tej-cli/            # CLI application
│       └── src/
│           ├── main.rs     # Entry point
│           ├── display.rs  # Table output
│           └── output.rs   # JSON output
└── apps/
    └── tej-gui/            # Tauri GUI application
        ├── src-tauri/      # Rust backend
        └── src/            # Svelte frontend
```

## Coding Guidelines

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- All public items must have documentation
- Prefer `thiserror` for error types
- Use `clippy` with `-D warnings`

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` new feature
- `fix:` bug fix
- `docs:` documentation changes
- `test:` adding/updating tests
- `refactor:` code refactoring
- `chore:` maintenance tasks

### Testing

- Write unit tests for new functionality
- Place tests in the same file using `#[cfg(test)]`
- Integration tests go in `tests/` directory
- Aim for good coverage of edge cases

## Release Process

1. Update version in `Cargo.toml` (workspace)
2. Update CHANGELOG.md
3. Create a git tag: `git tag v0.1.0`
4. Push tag: `git push origin v0.1.0`
5. CI will automatically build and publish releases

## Getting Help

- Open a [Discussion](https://github.com/pm-bhatt/tej/discussions) for questions
- Join our community chat (coming soon)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
