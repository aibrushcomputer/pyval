# Installation

## pip

```bash
pip install emailval
```

## Requirements

- Python 3.9 or later
- No other dependencies required

## Platform Support

Pre-built wheels available for:
- Linux (x86_64, aarch64)
- macOS (x86_64, arm64)
- Windows (x86_64)

## Building from Source

If a pre-built wheel is not available for your platform:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install from source
pip install emailval --no-binary :all:
```

Requires Rust 1.70 or later.
