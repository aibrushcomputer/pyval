# Contributing

## Development Setup

```bash
# Clone the repository
git clone https://github.com/aibrushcomputer/pyval.git
cd pyval

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Create virtual environment
python -m venv .venv
source .venv/bin/activate

# Install Python dependencies
pip install maturin pytest email-validator

# Build and install
make dev
```

## Branch Workflow

We use a branch-based workflow:

1. Create a feature branch from `develop`:
   ```bash
   git checkout develop
   git checkout -b feature/my-feature
   ```

2. Make changes and commit:
   ```bash
   git add .
   git commit -m "Add: description"
   git push origin feature/my-feature
   ```

3. Create a Pull Request to `develop`

4. After review and CI passes, merge to `develop`

5. When ready to release, merge `develop` to `main`:
   - Update version in `pyproject.toml` and `Cargo.toml`
   - Push to main triggers automatic release

## Running Tests

```bash
# Rust tests
cargo test --all

# Python tests
make test

# Benchmarks
make bench
```

## Code Style

- Rust: `cargo fmt` and `cargo clippy`
- Python: Follow PEP 8

## Reporting Issues

Please use [GitHub Issues](https://github.com/aibrushcomputer/pyval/issues) for bug reports and feature requests.
