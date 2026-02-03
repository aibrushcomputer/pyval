# Makefile for pyval development

.PHONY: help build test bench clean format lint doc

help:
	@echo "Available targets:"
	@echo "  build    - Build the Python package (release mode)"
	@echo "  dev      - Build and install in development mode"
	@echo "  test     - Run all tests"
	@echo "  bench    - Run performance benchmarks"
	@echo "  clean    - Clean build artifacts"
	@echo "  format   - Format Rust and Python code"
	@echo "  lint     - Run linters"
	@echo "  doc      - Build documentation"

build:
	cd wrappers/python && maturin build --release

dev:
	cd wrappers/python && maturin develop --release

test: dev
	pytest wrappers/python/tests/ -v

bench: dev
	python wrappers/python/tests/test_performance.py

clean:
	cd wrappers/python && cargo clean
	rm -rf wrappers/python/target
	rm -rf wrappers/python/build
	rm -rf wrappers/python/dist
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	find . -type f -name "*.pyc" -delete 2>/dev/null || true

format:
	cd crates/pyval-core && cargo fmt
	cd wrappers/python && cargo fmt
	black wrappers/python/pyval wrappers/python/tests 2>/dev/null || true

lint:
	cd crates/pyval-core && cargo clippy -- -D warnings
	cd wrappers/python && cargo clippy -- -D warnings

doc:
	cd crates/pyval-core && cargo doc --no-deps
