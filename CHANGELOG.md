# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub CI/CD workflows for testing and releasing
- Documentation structure with guides and API reference
- Workspace structure for multi-language support

## [0.2.0] - 2024-02-03

### Added
- **Major performance improvements**: 100-500x speedup over alternatives
- `is_valid_ultra()` function for maximum performance
- `batch_is_valid()` for efficient batch validation
- Lookup tables for O(1) character validation
- SWAR (SIMD Within A Register) optimizations
- Zero-copy validation path
- Fast ASCII email check

### Changed
- Restructured as Cargo workspace
- Separated core library from Python wrapper
- Improved error messages

### Removed
- Dead code and experimental modules
- TODO comments

## [0.1.0] - 2024-02-01

### Added
- Initial implementation
- Basic email validation
- Python bindings via PyO3
- RFC 5322/6531 compliance
- IDN support
- Test suite
- Benchmark suite

[Unreleased]: https://github.com/aibrushcomputer/pyval/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/aibrushcomputer/pyval/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/aibrushcomputer/pyval/releases/tag/v0.1.0
