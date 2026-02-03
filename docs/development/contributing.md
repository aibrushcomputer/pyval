# Contributing to pyval

Thank you for your interest in contributing to pyval! This guide will help you get started.

## Development Setup

### Prerequisites

- Python 3.8+
- Rust 1.70+
- maturin: `pip install maturin`

### Building from Source

```bash
# Clone the repository
git clone https://github.com/aibrushcomputer/pyval.git
cd pyval

# Build the Python wrapper
cd wrappers/python
maturin develop --release

# Run tests
pytest tests/
```

### Project Structure

```
pyval/
├── crates/pyval-core/     # Core Rust library
│   └── src/
├── wrappers/
│   └── python/            # Python wrapper
│       ├── src/           # PyO3 bindings
│       └── tests/         # Python tests
└── docs/                  # Documentation
```

## Making Changes

### Rust Code

1. Core validation logic goes in `crates/pyval-core/src/`
2. Keep functions small and focused
3. Add `#[inline]` for small hot-path functions
4. Run `cargo fmt` and `cargo clippy` before committing

### Python Code

1. Python-specific code goes in `wrappers/python/`
2. Add type hints where possible
3. Update docstrings for public APIs

### Tests

- Add unit tests for new functionality
- Ensure all tests pass: `pytest tests/`
- Run benchmarks: `python tests/test_performance.py`

## Submitting Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `pytest tests/`
5. Commit with clear messages
6. Push and create a Pull Request

## Code Style

### Rust

- Follow standard Rust style (enforced by `cargo fmt`)
- Add `#[inline(always)]` for very small hot functions
- Document public APIs with `///` comments

### Python

- Follow PEP 8
- Use type hints
- Docstrings in Google style

## Reporting Issues

- Use GitHub Issues
- Include Python version and OS
- Provide minimal reproduction code
- Include benchmark results if performance-related

## Questions?

Feel free to open an issue for questions or join our discussions!
