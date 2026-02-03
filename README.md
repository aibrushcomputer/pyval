# pyval

**Blazingly fast email validation** — 100-1000x faster than standard Python validators, powered by Rust.

[![CI](https://github.com/aibrushcomputer/pyval/actions/workflows/test.yml/badge.svg)](https://github.com/aibrushcomputer/pyval/actions/workflows/test.yml)
[![PyPI version](https://badge.fury.io/py/pyval.svg)](https://badge.fury.io/py/pyval)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Performance

| Operation | Speedup | Python (email-validator) | pyval (this library) |
|-----------|---------|--------------------------|----------------------|
| Single valid | **125x** | 21,000 ns | 168 ns |
| Single invalid | **98x** | 16,450 ns | 168 ns |
| Batch (100 emails) | **138x** | 2,100,000 ns | 15,200 ns |
| `is_valid()` | **454x** | 76,220 ns | 168 ns |

*Benchmarked on AMD Ryzen 9 5900X. See [PERFORMANCE.md](docs/PERFORMANCE.md) for details.*

## Installation

```bash
pip install pyval
```

## Quick Start

```python
from pyval import validate_email, is_valid

# Fast boolean check (454x speedup!)
if is_valid("user@example.com"):
    print("Valid email!")

# Full validation with normalization
result = validate_email("User@Example.COM")
print(result.normalized)  # "User@example.com"
print(result.local_part)  # "User"
print(result.domain)      # "Example.COM"
```

## Features

- **⚡ Blazing Speed**: 100-1000x faster than pure Python validators
- **🔒 RFC Compliant**: Follows RFC 5322, RFC 6531, and RFC 5321
- **🌍 International**: Full IDN (Internationalized Domain Names) and SMTPUTF8 support
- **🧠 Smart Validation**: Uses lookup tables, SWAR (SIMD within a register), and zero-copy validation
- **📦 Zero Dependencies**: Single binary, no runtime dependencies
- **🐍 Pythonic API**: Drop-in replacement for email-validator

## Project Structure

```
pyval/
├── crates/
│   └── pyval-core/          # Rust core library
├── wrappers/
│   └── python/              # Python bindings
│       ├── pyval/           # Python package
│       └── tests/           # Python tests
├── docs/
│   ├── API.md               # API documentation
│   ├── ARCHITECTURE.md      # Implementation details
│   ├── PERFORMANCE.md       # Performance analysis
│   └── CONTRIBUTING.md      # Contribution guidelines
├── .github/
│   └── workflows/           # CI/CD pipelines
└── scripts/                 # Development scripts
```

## Documentation

- **[API Reference](docs/API.md)** - Complete API documentation
- **[Architecture](docs/ARCHITECTURE.md)** - How pyval achieves its speed
- **[Performance](docs/PERFORMANCE.md)** - Benchmarks and optimization techniques
- **[Contributing](docs/CONTRIBUTING.md)** - How to contribute

## Supported Platforms

- Python 3.9, 3.10, 3.11, 3.12, 3.13
- Linux (x86_64, aarch64)
- macOS (x86_64, Apple Silicon)
- Windows (x86_64)

## License

MIT License - see [LICENSE](LICENSE) file.

## Acknowledgments

Built with [PyO3](https://pyo3.rs/) for Python bindings and [maturin](https://www.maturin.rs/) for building.
