# Tej Web Speed Test

Web-based speed test using WebAssembly (WASM) and Oat UI. Runs entirely in the browser with zero backend costs.

## Features

- **Honest measurements**: Uses incompressible random data (unlike Ookla/Fast.com)
- **Comprehensive metrics**: Download, upload, latency, jitter, packet loss
- **Lightweight**: ~8KB HTML/CSS + WASM bundle
- **No telemetry**: No data collection, runs entirely in your browser
- **Comparison table**: Side-by-side comparison with Ookla and Fast.com

## Architecture

```
Browser
├── Oat UI (HTML/CSS/JS) - 8KB
├── tej-core.wasm (Rust) - Speed test logic
└── Cloudflare Speed Test endpoints
```

## Building

### Prerequisites

- Rust with wasm32 target: `rustup target add wasm32-unknown-unknown`
- wasm-pack: `cargo install wasm-pack`

### Build

```bash
./build-web.sh
```

Or manually:

```bash
cd crates/tej-core
wasm-pack build --target web --out-dir ../../apps/tej-web/pkg
```

### Local Testing

```bash
cd apps/tej-web
python3 -m http.server 8000
# Open http://localhost:8000
```

## Deployment

### Netlify (Recommended - Free)

1. Push code to GitHub
2. Connect repository to Netlify
3. Build settings:
   - Build command: `cd crates/tej-core && wasm-pack build --target web --out-dir ../../apps/tej-web/pkg`
   - Publish directory: `apps/tej-web`
4. Deploy!

### GitHub Pages (Free)

1. Build locally: `./build-web.sh`
2. Push `apps/tej-web/` to `gh-pages` branch
3. Enable GitHub Pages in repository settings

## Files

- `index.html` - Main page with Oat UI
- `app.js` - JavaScript logic, WASM integration
- `pkg/` - Generated WASM files (created by wasm-pack)

## Comparison with Other Speed Tests

| Feature | Tej | Ookla | Fast.com |
|---------|-----|-------|----------|
| Incompressible data | ✓ | ✗ | ✗ |
| Open source | ✓ | ✗ | ✗ |
| No telemetry | ✓ | ✗ | ✗ |
| Lightweight | ✓ (~8KB) | ✗ (~200MB) | Web only |

## License

MIT - See main project LICENSE
